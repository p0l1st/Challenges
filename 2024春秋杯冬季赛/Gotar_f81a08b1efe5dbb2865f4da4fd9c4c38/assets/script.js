// assets/script.js
document.addEventListener('DOMContentLoaded', function() {
    const uploadForm = document.getElementById('uploadForm');

    uploadForm.addEventListener('submit', function(e) {
        e.preventDefault();

        const formData = new FormData(uploadForm);
        fetch('/upload', {
            method: 'POST',
            body: formData,
        })
            .then(response => {
                if (response.ok) {
                    alert("Upload successful!");
                    window.location.reload();
                } else {
                    alert("Upload failed.");
                }
            })
            .catch(error => {
                alert("An error occurred: " + error);
            });
    });
});