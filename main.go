package main

import (
	"fmt"
)

type charStruct struct {
	char       string
	loopNumber int
}

func main() {
	text := "aaabbcdddd"
	result := compressText(text)
	fmt.Println(result)
}

func compressText(text string) string {
	textArray := []rune(text)
	var previousChar rune
	equalCharLoopNumber := 1
	result := []charStruct{}
	for i := range textArray {
		// 이전 문자랑 현재 확인하는 문자랑 같으면 같은 문자 반복된거니까 equalCharLoopNumber++
		if textArray[i] == previousChar {
			equalCharLoopNumber++
		} else { // 이전 문자랑 현재 루프에서 확인하는 문자랑 다를 때
			// 첫번째 이전 문자는 존재하지 않기 때문에 첫번째로 시작되는걸(previousChar가 0으로 자동 초기화됨) 무시함
			if previousChar != 0 {
				result = append(result, charStruct{char: string(previousChar), loopNumber: equalCharLoopNumber})
				equalCharLoopNumber = 1
			}
			previousChar = textArray[i] // 루프에서 확인하는 문자가 다르니까 이제 새롭게 이전 문자 변수를 현재 확인 중인 문자로 갱신
		}
	}

	// 다음 문자가 바뀌면 이전 문자를 처리하는 식으로 한칸씩 밀리기 때문에 마지막은 수동으로 처리
	result = append(result, charStruct{char: string(previousChar), loopNumber: equalCharLoopNumber})

	convertedText := convertText(result)
	return convertedText
}

func convertText(result []charStruct) string {
	var convertedText string

	for i := range result {
		if result[i].loopNumber != 1 {
			convertedText = fmt.Sprintf("%s%s%d", convertedText, result[i].char, result[i].loopNumber)
		} else {
			convertedText = fmt.Sprintf("%s%s", convertedText, result[i].char)
		}
	}
	return convertedText
}
