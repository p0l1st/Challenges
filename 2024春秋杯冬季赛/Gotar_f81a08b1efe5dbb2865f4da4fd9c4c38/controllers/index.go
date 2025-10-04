// controllers/index.go
package controllers

import (
	"Gotar/db"
	"Gotar/models"
	"html/template"
	"net/http"
)

func IndexHandler(w http.ResponseWriter, r *http.Request) {
	_, err := r.Cookie("token")
	if err != nil {
		http.Redirect(w, r, "/login", http.StatusSeeOther)
		return
	}

	userID := r.Context().Value("userID").(uint)

	var user models.User
	result := db.DB.First(&user, userID)
	if result.Error != nil {
		http.Error(w, "User not found", http.StatusNotFound)
		return
	}

	tmpl := template.Must(template.ParseFiles("assets/index.html"))
	tmpl.Execute(w, map[string]interface{}{
		"User": user,
	})
}
