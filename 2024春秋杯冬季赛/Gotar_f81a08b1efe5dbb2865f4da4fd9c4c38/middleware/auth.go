package middleware

import (
	"Gotar/config"
	"Gotar/utils"
	"context"
	"github.com/golang-jwt/jwt"
	"net/http"
)

func AuthMiddleware(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		cookie, err := r.Cookie("token")
		if err != nil {
			http.Redirect(w, r, "/logout", http.StatusSeeOther)
			return
		}

		tokenStr := cookie.Value
		claims := &utils.Claims{}

		token, err := jwt.ParseWithClaims(tokenStr, claims, func(token *jwt.Token) (interface{}, error) {
			return config.JWTKey, nil
		})
		if err != nil {
			if err == jwt.ErrSignatureInvalid {
				http.Error(w, "Unauthorized", http.StatusUnauthorized)
				return
			}
			http.Error(w, "Bad request! Hacker!!!", http.StatusBadRequest)
			return
		}
		if !token.Valid {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		ctx := context.WithValue(r.Context(), "userID", claims.UserID)
		ctx = context.WithValue(ctx, "isAdmin", claims.IsAdmin)
		next.ServeHTTP(w, r.WithContext(ctx))
	}
}
