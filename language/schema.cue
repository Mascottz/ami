package language

#Pair: close({
	key: string
	lower: string
	upper: string
	feature: "hook" | "subdot" | "overdot"
})

#Language: close({
	code: "ha" | "ig" | "pcm"
	label: string
	hooked: [...#Pair]
	subdots: [...#Pair]
	overdots: [...#Pair]
	toneSamples: [...string]
	tonePolicy: "optional" | "none"
	pidginPolicy: "descriptive" | "not-applicable"
})

#Hausa: #Language & {
	code: "ha"
	label: "Hausa"
	hooked: [
		{key: "b", lower: "ɓ", upper: "Ɓ", feature: "hook"},
		{key: "d", lower: "ɗ", upper: "Ɗ", feature: "hook"},
		{key: "k", lower: "ƙ", upper: "Ƙ", feature: "hook"},
		{key: "y", lower: "ƴ", upper: "Ƴ", feature: "hook"},
	]
	subdots: []
	overdots: []
	toneSamples: ["à", "á", "â"]
	tonePolicy: "optional"
	pidginPolicy: "not-applicable"
}

#Igbo: #Language & {
	code: "ig"
	label: "Igbo"
	hooked: []
	subdots: [
		{key: "i", lower: "ị", upper: "Ị", feature: "subdot"},
		{key: "o", lower: "ọ", upper: "Ọ", feature: "subdot"},
		{key: "u", lower: "ụ", upper: "Ụ", feature: "subdot"},
	]
	overdots: [
		{key: "n", lower: "ṅ", upper: "Ṅ", feature: "overdot"},
	]
	toneSamples: ["á", "à", "í", "ì", "ị́", "ị̀", "ọ́", "ọ̀", "ụ́", "ụ̀"]
	tonePolicy: "optional"
	pidginPolicy: "not-applicable"
}

#Pidgin: #Language & {
	code: "pcm"
	label: "Naija Pidgin"
	hooked: []
	subdots: []
	overdots: []
	toneSamples: []
	tonePolicy: "none"
	pidginPolicy: "descriptive"
}

languages: [#Hausa, #Igbo, #Pidgin]
