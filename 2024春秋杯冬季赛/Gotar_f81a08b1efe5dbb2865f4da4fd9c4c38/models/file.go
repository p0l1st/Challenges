// models/file.go
package models

import (
	"gorm.io/gorm"
)

type File struct {
	gorm.Model
	UserID        uint
	Name          string
	Path          string
	ExtractedPath string // New field for extracted file path
}
