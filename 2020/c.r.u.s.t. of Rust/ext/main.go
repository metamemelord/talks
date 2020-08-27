package main

import (
	"math/rand"
	"sync"
)

func worker(m map[string]int, wg *sync.WaitGroup) {
	key := randString(1)
	if _, ok := m[key]; !ok {
		m[key] = 0
	}
	m[key]++
	wg.Done()
}

func randString(s int) string {
	lowercase := "abcdefghijklmnopqrstunwxyz"
	result := []byte{}
	for len(result) != s {
		 result = append(result, lowercase[rand.Intn(26)])
	}
	return string(result)
}

func main() {
	m := make(map[string]int)
	var wg sync.WaitGroup
	for i:=0;i<1000;i++ {
		wg.Add(1)
		go worker(m, &wg);
	}
	wg.Wait()
}