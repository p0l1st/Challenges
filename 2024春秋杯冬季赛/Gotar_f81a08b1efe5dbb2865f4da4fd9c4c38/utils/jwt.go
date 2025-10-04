// utils/jwt.go
package utils

import (
	"github.com/golang-jwt/jwt"
	"time"
)

type Claims struct {
	UserID  uint
	IsAdmin bool
	jwt.StandardClaims
}

func GenerateJWT(userID uint, isAdmin bool, jwtKey []byte) (string, error) {
	expirationTime := time.Now().Add(24 * time.Hour)
	claims := &Claims{
		UserID:  userID,
		IsAdmin: isAdmin,
		StandardClaims: jwt.StandardClaims{
			ExpiresAt: expirationTime.Unix(),
		},
	}

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	return token.SignedString(jwtKey)
}
