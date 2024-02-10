<?php
$uploadDir = 'uploads/'; 
error_log(0);
$files = scandir($uploadDir);
foreach ($files as $file) {
    if ($file != "." && $file != "..") {
        $filePath = $uploadDir . $file;
        $fileLastModified = filemtime($filePath);
        $currentTime = time();
        $fileExistTime = $currentTime - $fileLastModified;
        if ($fileExistTime > 10) { //
            unlink($filePath);
        }
    }
}

if (isset($_FILES['file'])) {
    $allowedFileTypes = array('jpg', 'jpeg', 'png', 'gif',"php");
    $fileName = $_FILES['file']['name'];
    $tempFile = $_FILES['file']['tmp_name'];
    $fileHeader = finfo_file(finfo_open(FILEINFO_MIME_TYPE), $tempFile);
    $fileType = explode('/', $fileHeader)[1];
    if (in_array($fileType, $allowedFileTypes)) {
        $targetFile = $uploadDir . $fileName;
        if (move_uploaded_file($tempFile, $targetFile)) {
            $fileContent = file_get_contents($targetFile);
            if (strpos($fileContent, '<?php') === false) {
                echo '文件上传成功。';
            } else {
                unlink($targetFile);
                echo '文件上传失败，不支持的文件类型';
            }
        } else {
            echo '文件上传失败。';
        }
    } else {
        echo '不支持的文件类型。';
    }
} else {
    echo '未收到有效的文件上传。';
}
?>
