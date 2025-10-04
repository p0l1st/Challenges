package main

import (
	"Gotar/config"
	"Gotar/controllers"
	"Gotar/db"
	"Gotar/middleware"
	"log"
	"net/http"
)

func main() {
	config.LoadEnv()

	db.Init()

	http.Handle("/assets/", http.StripPrefix("/assets/", http.FileServer(http.Dir("assets"))))
	http.HandleFunc("/", middleware.AuthMiddleware(controllers.IndexHandler))
	http.HandleFunc("/register", controllers.RegisterHandler)
	http.HandleFunc("/login", controllers.LoginHandler)
	http.HandleFunc("/logout", controllers.LogoutHandler)
	http.HandleFunc("/upload", middleware.AuthMiddleware(controllers.UploadHandler))
	http.HandleFunc("/files", middleware.AuthMiddleware(controllers.FilesHandler))
	http.HandleFunc("/download/", middleware.AuthMiddleware(controllers.DownloadHandler))

	log.Println("Server started on http://localhost:8081")
	log.Fatal(http.ListenAndServe(":80", nil))
}
