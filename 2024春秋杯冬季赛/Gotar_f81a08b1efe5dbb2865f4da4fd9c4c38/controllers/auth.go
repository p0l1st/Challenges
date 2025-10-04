// controllers/auth.go
package controllers

import (
	"Gotar/config"
	"Gotar/db"
	"Gotar/models"
	"Gotar/utils"
	"golang.org/x/crypto/bcrypt"
	"gorm.io/gorm"
	"html/template"
	"net/http"
)

func RegisterHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method == http.MethodPost {
		username := r.FormValue("username")
		password := r.FormValue("password")

		hashedPassword, err := bcrypt.GenerateFromPassword([]byte(password), bcrypt.DefaultCost)
		if err != nil {
			http.Error(w, "Failed to hash password", http.StatusInternalServerError)
			return
		}

		user := models.User{Username: username, Password: string(hashedPassword)}
		result := db.DB.Create(&user)
		if result.Error != nil {
			http.Error(w, "Failed to create user", http.StatusInternalServerError)
			return
		}

		// Redirect to login page after successful registration
		http.Redirect(w, r, "/login", http.StatusSeeOther)
		return
	}

	tmpl := template.Must(template.ParseFiles("assets/register.html"))
	tmpl.Execute(w, nil)
}

func LoginHandler(w http.ResponseWriter, r *http.Request) {
	config.LoadEnv()
	if r.Method == http.MethodPost {
		username := r.FormValue("username")
		password := r.FormValue("password")

		var user models.User
		result := db.DB.Where("username = ?", username).First(&user)
		if result.Error != nil {
			if result.Error == gorm.ErrRecordNotFound {
				http.Error(w, "Invalid username or password", http.StatusUnauthorized)
				return
			}
			http.Error(w, "Failed to query user", http.StatusInternalServerError)
			return
		}

		err := bcrypt.CompareHashAndPassword([]byte(user.Password), []byte(password))
		if err != nil {
			http.Error(w, "Invalid username or password", http.StatusUnauthorized)
			return
		}

		token, err := utils.GenerateJWT(user.ID, user.IsAdmin, config.JWTKey)
		if err != nil {
			http.Error(w, "Failed to generate token", http.StatusInternalServerError)
			return
		}

		http.SetCookie(w, &http.Cookie{
			Name:  "token",
			Value: token,
			Path:  "/",
		})

		http.Redirect(w, r, "/", http.StatusSeeOther)
		return
	}

	tmpl := template.Must(template.ParseFiles("assets/login.html"))
	tmpl.Execute(w, nil)
}

func LogoutHandler(w http.ResponseWriter, r *http.Request) {
	// Clear the token cookie
	http.SetCookie(w, &http.Cookie{
		Name:   "token",
		Value:  "",
		Path:   "/",
		MaxAge: -1, // This will delete the cookie
	})

	http.Redirect(w, r, "/login", http.StatusSeeOther)
}
