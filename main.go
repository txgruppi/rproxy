package main

import (
	"bytes"
	"crypto/aes"
	"crypto/cipher"
	"encoding/base64"
	"fmt"
	"io"
	"strings"

	"github.com/davecgh/go-spew/spew"
)

func main() {
	key := []byte("12345678123456781234567812345678")
	data := []byte("{}")
	fmt.Println(aesEncrypt(key, data))
}

func newTestIV() io.Reader {
	return strings.NewReader("1234567812345678")
}

func aesEncrypt(key []byte, input []byte) (encoded string, err error) {
	block, err := aes.NewCipher(key)
	if err != nil {
		return "", err
	}

	payload := zeroPad(input, aes.BlockSize)
	spew.Dump(payload)
	output := make([]byte, aes.BlockSize+len(payload))

	// CBC mode works on blocks, so plain text may need to be padded to the next whole block
	// https://www.rfc-editor.org/rfc/rfc5246#section-6.2.3.2
	iv := output[:aes.BlockSize]
	if _, err := io.ReadFull(newTestIV(), iv); err != nil {
		return "", err
	}

	mode := cipher.NewCBCEncrypter(block, iv)
	mode.CryptBlocks(output[aes.BlockSize:], payload)

	return base64.URLEncoding.EncodeToString(output), err
}

func withPadding(payload []byte, blockSize int) []byte {
	if len(payload)%aes.BlockSize == 0 {
		return payload
	}
	data := make([]byte, int(len(payload)/blockSize+1)*blockSize)
	copy(data, payload)
	return data
}

func pkcs5Padding(payload []byte, blockSize int) []byte {
	padding := blockSize - len(payload)%blockSize
	text := bytes.Repeat([]byte{byte(padding)}, padding)
	return append(payload, text...)
}

func zeroPad(payload []byte, blockSize int) []byte {
	padding := blockSize - len(payload)%blockSize
	text := bytes.Repeat([]byte{byte('0')}, padding)
	return append(payload, text...)
}
