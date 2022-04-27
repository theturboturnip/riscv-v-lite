	.text
	.attribute	4, 16
	.attribute	5, "rv32i2p0_m2p0_v0p10_zvlsseg0p10"
	.file	"hello_world.c"
	.globl	factorial
	.p2align	2
	.type	factorial,@function
factorial:
	addi	sp, sp, -16
	sw	ra, 12(sp)
	sw	s0, 8(sp)
	beqz	a0, .LBB0_2
	mv	s0, a0
	addi	a0, a0, -1
	call	factorial
	mul	a0, a0, s0
	j	.LBB0_3
.LBB0_2:
	addi	a0, zero, 1
.LBB0_3:
	lw	s0, 8(sp)
	lw	ra, 12(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end0:
	.size	factorial, .Lfunc_end0-factorial

	.globl	fac_test
	.p2align	2
	.type	fac_test,@function
fac_test:
	addi	sp, sp, -16
	sw	ra, 12(sp)
	addi	a0, zero, 10
	call	factorial
	lui	a1, 886
	addi	a1, a1, -256
	xor	a0, a0, a1
	seqz	a0, a0
	lw	ra, 12(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end1:
	.size	fac_test, .Lfunc_end1-fac_test

	.globl	fibbonacci
	.p2align	2
	.type	fibbonacci,@function
fibbonacci:
	addi	sp, sp, -16
	sw	ra, 12(sp)
	sw	s0, 8(sp)
	sw	s1, 4(sp)
	mv	s0, a0
	beqz	a0, .LBB2_3
	addi	a0, zero, 1
	beq	s0, a0, .LBB2_4
	addi	a0, s0, -1
	call	fibbonacci
	mv	s1, a0
	addi	a0, s0, -2
	call	fibbonacci
	add	a0, a0, s1
	j	.LBB2_4
.LBB2_3:
	mv	a0, s0
.LBB2_4:
	lw	s1, 4(sp)
	lw	s0, 8(sp)
	lw	ra, 12(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end2:
	.size	fibbonacci, .Lfunc_end2-fibbonacci

	.globl	fib_test
	.p2align	2
	.type	fib_test,@function
fib_test:
	addi	sp, sp, -16
	sw	ra, 12(sp)
	addi	a0, zero, 10
	call	fibbonacci
	addi	a0, a0, -55
	seqz	a0, a0
	lw	ra, 12(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end3:
	.size	fib_test, .Lfunc_end3-fib_test

	.globl	fib_memo
	.p2align	2
	.type	fib_memo,@function
fib_memo:
	addi	sp, sp, -208
	mv	a1, zero
	addi	a2, sp, 8
	addi	a3, zero, 1
	addi	a6, zero, 50
	j	.LBB4_3
.LBB4_1:
	lw	a5, -4(a2)
	lw	a4, -8(a2)
	add	a4, a4, a5
	sw	a4, 0(a2)
.LBB4_2:
	addi	a1, a1, 1
	addi	a2, a2, 4
	beq	a1, a6, .LBB4_7
.LBB4_3:
	beq	a1, a3, .LBB4_6
	bnez	a1, .LBB4_1
	sw	zero, 8(sp)
	j	.LBB4_2
.LBB4_6:
	sw	a3, 12(sp)
	j	.LBB4_2
.LBB4_7:
	slli	a0, a0, 2
	addi	a1, sp, 8
	add	a0, a1, a0
	lw	a0, 0(a0)
	addi	sp, sp, 208
	ret
.Lfunc_end4:
	.size	fib_memo, .Lfunc_end4-fib_memo

	.globl	fib_memo_test
	.p2align	2
	.type	fib_memo_test,@function
fib_memo_test:
	addi	sp, sp, -16
	sw	ra, 12(sp)
	addi	a0, zero, 33
	call	fib_memo
	lui	a1, 860
	addi	a1, a1, 2018
	xor	a0, a0, a1
	seqz	a0, a0
	lw	ra, 12(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end5:
	.size	fib_memo_test, .Lfunc_end5-fib_memo_test

	.globl	main
	.p2align	2
	.type	main,@function
main:
	addi	sp, sp, -16
	sw	ra, 12(sp)
	sw	s0, 8(sp)
	call	fac_test
	mv	s0, a0
	call	fib_test
	slli	a0, a0, 1
	or	s0, a0, s0
	call	fib_memo_test
	slli	a0, a0, 2
	or	a0, s0, a0
	lui	a1, 983040
	addi	a2, zero, 7
	sw	a2, 0(a1)
	sw	a0, 8(a1)
	lw	s0, 8(sp)
	lw	ra, 12(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end6:
	.size	main, .Lfunc_end6-main

	.ident	"Ubuntu clang version 13.0.1-++20220120110924+75e33f71c2da-1~exp1~20220120231001.58"
	.section	".note.GNU-stack","",@progbits
	.addrsig
