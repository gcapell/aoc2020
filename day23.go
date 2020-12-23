package main

import "fmt"

const maxCup = 1_000_000

type Cup struct {
	n    int
	l, r *Cup
}

func main() {
	fmt.Println("hello")

	c, l2c := cups("253149867")

	for j := 0; j < 10_000_000; j++ {
		c = round(c, l2c)
		//printCircle(c)
	}
	cup1 := l2c[1]
	fmt.Println(cup1.r.n * cup1.r.r.n)
}

func printCircle(h *Cup) {
	c := h
	for {
		fmt.Printf("%d,", c.n)
		c = c.r
		if c == h {
			fmt.Println()
			return
		}
	}
}

func round(c *Cup, labelToCup []*Cup) *Cup {
	// snip
	h := c.r
	t := c.r.r.r
	c.r = t.r
	t.r.l = c

	dst := labelToCup[findLabel(c.n, h)]

	// insert
	t.r = dst.r
	h.l = dst
	dst.r.l = t
	dst.r = h

	return c.r
}

func findLabel(n int, h *Cup) int {
	for {
		n--
		if n < 1 {
			n = 1_000_000
		}
		if !(n == h.n || n == h.r.n || n == h.r.r.n) {
			return n
		}
	}
}

func cups(s string) (*Cup, []*Cup) {
	var head, tail *Cup
	l2c := make([]*Cup, maxCup+1)

	for _, n := range s {
		v := int(n - '0')
		c := &Cup{n: v}
		l2c[v] = c
		if head == nil {
			head = c
			tail = c
			c.l = c
			c.r = c
		} else {
			tail.r = c
			c.l = tail
			c.r = head
			head.l = c
			tail = c
		}
	}
	for j := 10; j <= maxCup; j++ {
		c := &Cup{n: j}
		l2c[j] = c
		tail.r = c
		c.l = tail
		c.r = head
		head.l = c
		tail = c
	}
	return head, l2c
}
