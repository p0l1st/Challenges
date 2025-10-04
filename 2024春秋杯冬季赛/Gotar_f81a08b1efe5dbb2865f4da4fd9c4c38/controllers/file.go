package controllers

import (
	"Gotar/db"
	"Gotar/models"
	"fmt"
	"github.com/whyrusleeping/tar-utils"
	"html/template"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

const (
	uploadDir    = "./assets/uploads"
	extractedDir = "./assets/extracted"
)

func UploadHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Invalid request method", http.StatusMethodNotAllowed)
		return
	}

	err := r.ParseMultipartForm(10 << 20) // 10MB limit
	if err != nil {
		http.Error(w, "Failed to parse form", http.StatusBadRequest)
		return
	}

	file, header, err := r.FormFile("file")
	if err != nil {
		http.Error(w, "Failed to retrieve file", http.StatusBadRequest)
		return
	}
	defer file.Close()

	userID := r.Context().Value("userID").(uint)
	filePath := filepath.Join(uploadDir, fmt.Sprintf("%d_%s", userID, header.Filename))
	outFile, err := os.Create(filePath)
	if err != nil {
		http.Error(w, "Failed to create file", http.StatusInternalServerError)
		return
	}
	defer outFile.Close()

	_, err = io.Copy(outFile, file)
	if err != nil {
		http.Error(w, "Failed to save file", http.StatusInternalServerError)
		return
	}
	extractedPath, err := extractTar(filePath, userID)
	if err != nil {
		http.Error(w, fmt.Sprintf("Failed to extract file: %v", err), http.StatusInternalServerError)
		return
	}

	fileRecord := models.File{
		UserID:        userID,
		Name:          header.Filename,
		Path:          filePath,
		ExtractedPath: extractedPath,
	}
	db.DB.Create(&fileRecord)

	http.Redirect(w, r, "/files", http.StatusSeeOther)
}

func FilesHandler(w http.ResponseWriter, r *http.Request) {
	var files []models.File

	db.DB.Find(&files)

	tmpl := template.Must(template.ParseFiles("assets/files.html"))
	tmpl.Execute(w, map[string]interface{}{
		"Files": files,
	})
}

func DownloadHandler(w http.ResponseWriter, r *http.Request) {
	userID := r.Context().Value("userID").(uint)
	fileID := strings.TrimSuffix(strings.TrimPrefix(r.URL.Path, "/download/"), "/")
	var file models.File

	result := db.DB.Where("id = ? AND user_id = ?", fileID, userID).First(&file)
	if result.Error != nil {
		http.Error(w, "File not found or access denied", http.StatusNotFound)
		return
	}

	http.ServeFile(w, r, file.ExtractedPath)
}

func extractTar(tarPath string, userID uint) (string, error) {
	userDir := filepath.Join(extractedDir, fmt.Sprintf("%d", userID))
	err := os.MkdirAll(userDir, os.ModePerm)
	if err != nil {
		return "", err
	}
	tarFile, err := os.Open(tarPath)
	if err != nil {
		return "", err
	}
	defer tarFile.Close()
	extractor := &tar.Extractor{
		Path: userDir,
	}
	err = extractor.Extract(tarFile)
	if err != nil {
		return "", err
	}
	return userDir, nil
}
