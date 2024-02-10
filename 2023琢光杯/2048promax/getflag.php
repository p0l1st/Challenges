<?php

function Hex2Bin($hexString) {
    $binaryString = "";
    for ($i = 0; $i < strlen($hexString); $i += 2) {
        $binaryString .= chr(hexdec(substr($hexString, $i, 2)));
    }
    return $binaryString;
}

function enhancedEnc($data, $key) {
    $str = "";

    $a = strrev(str_rot13($data));

    for ($i = 0; $i < strlen($a); $i++) {
        $b = ord($a[$i]) + ord($key[$i % strlen($key)]);
        $c = $b ^ 100;
        $e = sprintf("%02x", $c);
        $str .= $e;
    }

    return $str;
}

function enhancedDec($encryptedData, $key) {
    $str = "";

    $binaryData = Hex2Bin($encryptedData);

    for ($i = 0; $i < strlen($binaryData); $i++) {
        $c = ord($binaryData[$i]) ^ 100;
        $b = $c - ord($key[$i % strlen($key)]);
        $str .= chr($b);
    }

    $decryptedData = str_rot13(strrev($str));

    return $decryptedData;
}

// 例子：使用加密数据进行解密
$encryptedData = " \x57\x34\x65\x57\x57\x50\x31\x48\x75\x57  ";
$key = "94ad8786878f80ad84858d8f8fbe8fb88586bdbaa1d4a4aca4";

$decryptedData = enhancedDec($encryptedData, $key);
echo "Decrypted Data: " . $decryptedData . "\n";

?>
