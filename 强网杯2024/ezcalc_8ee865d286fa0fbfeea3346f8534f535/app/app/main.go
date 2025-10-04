package main

import (
	"bytes"
	"crypto/rand"
	"fmt"
	"io"
	"math/big"
	"net/http"
	"os"
	"path/filepath"
	"strings"
	"sync"

	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

var db *gorm.DB
var reportGoroutineCount int = 0
var reportMutex sync.Mutex

type ReportRequest struct {
	Expression  string   `json:"expression"`
	Result      string   `json:"result"`
	Email       string   `json:"email" binding:"required"`
	Comment     string   `json:"comment"`
	Screenshots []string `json:"screenshots"`
}

type Screenshot struct {
	ID       string `gorm:"type:uuid;primary_key" json:"id"`
	Path     string `gorm:"type:text" json:"path"`
	ReportID string `gorm:"type:uuid" json:"report_id"`
}

type Report struct {
	ID          string `gorm:"type:uuid;primary_key" json:"id"`
	Expression  string `gorm:"type:text" json:"expression"`
	Result      string `gorm:"type:text" json:"result"`
	Email       string `gorm:"type:text" json:"email"`
	Comment     string `gorm:"type:text" json:"comment"`
	CheckResult string `gorm:"type:text" json:"check_result"`
}

var blacklistedExt = [...]string{".htm", ".html", ".shtml", ".xhtml", ".mhtml", ".xht", ".chm"}
var letterRunes = []rune("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
var letterRunesLen = big.NewInt(int64(len(letterRunes)))

func RandStringRunes(n int) string {
	b := make([]rune, n)
	for i := range b {
		idx, err := rand.Int(rand.Reader, letterRunesLen)
		if err != nil {
			panic(err)
		}
		b[i] = letterRunes[idx.Int64()]
	}
	return string(b)
}

func StaticMiddleware(relativePath, root string) gin.HandlerFunc {
	return func(c *gin.Context) {
		if strings.HasPrefix(c.Request.URL.Path, relativePath) {
			fs := gin.Dir(root, false)
			fileServer := http.StripPrefix(relativePath, http.FileServer(fs))
			fileServer.ServeHTTP(c.Writer, c.Request)
		} else {
			c.Next()
		}
	}
}

func main() {
	var err error
	db, err = gorm.Open(sqlite.Open("database.db"), &gorm.Config{})
	if err != nil {
		panic("failed to connect database")
	}

	err = db.AutoMigrate(&Screenshot{}, &Report{})
	if err != nil {
		panic("failed to migrate database")
	}

	r := gin.Default()
	r.LoadHTMLGlob("templates/*")

	r.Use(func(c *gin.Context) {
		host := c.Request.Host
		if host == "" {
			c.JSON(http.StatusBadRequest, "Bad Request")
			c.Abort()
			return
		}
		nonce := RandStringRunes(32)
		c.Set("nonce", nonce)
		c.Header("Content-Security-Policy", fmt.Sprintf("default-src 'none'; script-src 'self' 'unsafe-eval' 'nonce-%s'; style-src 'unsafe-inline' 'self'; img-src 'self' data: blob:; connect-src 'self'; frame-src 'none'; base-uri 'none'; manifest-src 'none'; object-src 'none';", nonce))
		c.Header("X-Content-Type-Options", "nosniff")
		c.Header("X-Frame-Options", "DENY")
		c.Header("Cross-Origin-Opener-Policy", "same-origin")
		c.Next()
	})

	r.Use(StaticMiddleware("/static", "static"))

	r.POST("/api/screenshot/upload", func(c *gin.Context) {
		file, err := c.FormFile("file")
		if err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"status": "error", "message": "Invalid file"})
			return
		}

		if file.Size > 1*1024*1024 {
			c.JSON(http.StatusBadRequest, gin.H{"status": "error", "message": "File size too large"})
			return
		}

		dir := filepath.Join("static")

		if err := os.MkdirAll(dir, os.ModePerm); err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"status": "error", "message": "Internal Server Error"})
			return
		}

		id := uuid.New().String()
		ext := filepath.Ext(file.Filename)

		for _, item := range blacklistedExt {
			if strings.EqualFold(item, ext) {
				c.JSON(http.StatusBadRequest, gin.H{"status": "error", "message": "Not allowed to upload this file type"})
				return
			}
		}

		dst := filepath.Join(dir, id+ext)
		src, err := file.Open()
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"status": "error", "message": "Internal Server Error"})
			return
		}
		defer src.Close()
		data, err := io.ReadAll(src)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"status": "error", "message": "Internal Server Error"})
			return
		}

		contentType := http.DetectContentType(data)
		if !strings.HasPrefix(contentType, "image/") {
			c.JSON(http.StatusBadRequest, gin.H{"status": "error", "message": "Invalid file type"})
			return
		}

		out, err := os.Create(dst)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"status": "error", "message": "Internal Server Error"})
			return
		}
		defer out.Close()

		_, err = io.Copy(out, bytes.NewReader(data))
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"status": "error", "message": "Internal Server Error"})
			return
		}

		screenshot := Screenshot{
			ID:   id,
			Path: dst,
		}

		if err := db.Create(&screenshot).Error; err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"status": "error", "message": "Internal Server Error"})
			return
		}

		c.JSON(http.StatusOK, gin.H{"status": "ok", "data": screenshot})
	})

	r.DELETE("/api/screenshot/:id", func(c *gin.Context) {
		id := c.Param("id")

		var screenshot Screenshot
		if err := db.Where("id = ?", id).First(&screenshot).Error; err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"status": "error", "message": "Screenshot not found"})
			return
		}

		if screenshot.ReportID != "" {
			c.JSON(http.StatusBadRequest, gin.H{"status": "error", "message": "Screenshot associated with a report"})
			return
		}

		if err := db.Delete(&screenshot).Error; err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"status": "error", "message": "Internal Server Error"})
			return
		}

		os.Remove(screenshot.Path)

		c.JSON(http.StatusOK, gin.H{"status": "ok"})
	})

	r.POST("/api/report", func(c *gin.Context) {
		var req ReportRequest
		if err := c.BindJSON(&req); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"status": "error", "message": "Invalid request"})
			return
		}

		report := Report{
			ID:          uuid.New().String(),
			Expression:  req.Expression,
			Result:      req.Result,
			Email:       req.Email,
			Comment:     req.Comment,
			CheckResult: "waiting",
		}

		for _, id := range req.Screenshots {
			var screenshot Screenshot
			if err := db.Where("id = ?", id).First(&screenshot).Error; err != nil {
				c.JSON(http.StatusBadRequest, gin.H{"status": "error", "message": "Screenshot not found"})
				return
			}
			if screenshot.ReportID != "" {
				c.JSON(http.StatusBadRequest, gin.H{"status": "error", "message": "Screenshot already associated with a report"})
				return
			}

			screenshot.ReportID = report.ID
			if err := db.Save(&screenshot).Error; err != nil {
				c.JSON(http.StatusInternalServerError, gin.H{"status": "error", "message": "Internal Server Error"})
				return
			}
		}

		if err := db.Create(&report).Error; err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"status": "error", "message": "Internal Server Error"})
			return
		}

		reportMutex.Lock()
		if reportGoroutineCount >= 32 {
			reportMutex.Unlock()
			c.JSON(http.StatusTooManyRequests, gin.H{"status": "error", "message": "Too many requests"})
			return
		}
		reportMutex.Unlock()

		go func() {
			reportMutex.Lock()
			reportGoroutineCount++
			reportMutex.Unlock()
			defer func() {
				reportMutex.Lock()
				reportGoroutineCount--
				reportMutex.Unlock()
			}()

			req, err := http.NewRequest("GET", "http://bot:52000/api/bot", nil)
			if err != nil {
				report.CheckResult = "error"
				db.Save(&report)
				return
			}
			q := req.URL.Query()
			q.Add("expr", report.Expression)
			req.URL.RawQuery = q.Encode()

			client := &http.Client{}
			resp, err := client.Do(req)
			if err != nil {
				report.CheckResult = "error"
				db.Save(&report)
				return
			}

			defer resp.Body.Close()

			if resp.StatusCode != http.StatusOK {
				report.CheckResult = "error"
				db.Save(&report)
				return
			}

			resBody, err := io.ReadAll(resp.Body)
			if err != nil {
				report.CheckResult = "error"
				db.Save(&report)
				return
			}

			report.CheckResult = string(resBody)
			db.Save(&report)
		}()

		c.JSON(http.StatusOK, gin.H{"status": "ok", "data": gin.H{"id": report.ID}})
	})

	r.GET("/api/report/:id", func(c *gin.Context) {
		id := c.Param("id")

		var report Report
		if err := db.Where("id = ?", id).First(&report).Error; err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"status": "error", "message": "Report not found"})
			return
		}

		var screenshots []Screenshot
		if err := db.Where("report_id = ?", id).Find(&screenshots).Error; err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"status": "error", "message": "Internal Server Error"})
			return
		}

		c.JSON(http.StatusOK, gin.H{"status": "ok", "data": gin.H{"report": report, "screenshots": screenshots}})
	})

	r.NoRoute(func(c *gin.Context) {
		c.HTML(http.StatusOK, "index.tpl", gin.H{
			"nonce": c.MustGet("nonce"),
		})
	})

	r.Run(":9000")
}
