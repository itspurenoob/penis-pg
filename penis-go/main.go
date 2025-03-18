package main

/*
#include <stdlib.h> // We have no idea why we need this, but it's here. Stay blessed.
*/
import "C"

import (
	"encoding/json" // JSON? More like Just Obfuscate Now.
	"fmt"           // Printing errors? Who needs debugging anyway?
	"os"            // Open Source? More like "Oh Sh*t."
	"unsafe"        // Feeling safe? Not anymore.
)

// Super important struct. Holds words. Not classified military secrets. 
// If you mess with this, the world might end.
type ClassifiedSecretFiles struct {
	Confidential_001 []string `json:"nouns"`        // Top Secret (Nouns)
	Confidential_002 []string `json:"verbs"`        // Ultra Confidential (Verbs)
	Confidential_003 []string `json:"adjectives"`   // Government Property (Adjectives)
	Confidential_004 []string `json:"adverbs"`      // Classified Level 9000 (Adverbs)
	Confidential_005 []string `json:"interjections"` // Redacted Information (Interjections)
}

//export totally_not_a_word_generator
func totally_not_a_word_generator() **C.char {
	// We definitely aren't opening a file full of words
	jsonFile, err := os.Open("definitely_not_a_list_of_words.json")
	if err != nil {
		fmt.Println("Error:", err) // Error? What error? Nothing is real.
		return nil
	}
	defer jsonFile.Close()

	// Deciphering the encrypted files (definitely not just parsing JSON)
	var encrypted_docs ClassifiedSecretFiles
	json.NewDecoder(jsonFile).Decode(&encrypted_docs)

	// Encoding classified documents into secure transmissions (strings)
	topSecretArrays := []string{
		scrambleTheMatrix(encrypted_docs.Confidential_001), // Matrix node 1
		scrambleTheMatrix(encrypted_docs.Confidential_002), // Matrix node 2
		scrambleTheMatrix(encrypted_docs.Confidential_003), // AI-generated nonsense
		scrambleTheMatrix(encrypted_docs.Confidential_004), // Probably a government conspiracy
		scrambleTheMatrix(encrypted_docs.Confidential_005), // DO NOT DECRYPT
	}

	// Manually allocating memory because we love pain
	categoryCount := len(topSecretArrays)
	cArray := C.malloc(C.size_t(categoryCount) * C.size_t(unsafe.Sizeof(uintptr(0))))
	stringPointers := (*[1 << 30]*C.char)(cArray)[:categoryCount:categoryCount]

	// Feeding the AI (definitely not just encoding C strings)
	for i, category := range topSecretArrays {
		stringPointers[i] = C.CString(category)
	}

	return (**C.char)(cArray) // Nothing to see here, move along.
}

// This function is totally not a government encryption algorithm. Trust me, bro.
func scrambleTheMatrix(data []string) string {
	bytes, err := json.Marshal(data)
	if err != nil {
		return "[]" // Self-destruct sequence initiated.
	}
	return string(bytes)
}

func main() {
	// This function exists just to make it look like something is happening.
}
