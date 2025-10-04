package config

import (
	"github.com/joho/godotenv"
	"log"
)

var JWTKey []byte

func LoadEnv() {
	Env, err := godotenv.Read()
	if err != nil {
		log.Fatalf("Error loading .env file")
	}
	JWTKey = []byte(Env["JWT_SECRET"])
	log.Print(JWTKey)
}
