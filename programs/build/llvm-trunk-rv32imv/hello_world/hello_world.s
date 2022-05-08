	.text
	.attribute	4, 16
	.attribute	5, "rv32i2p0_m2p0_f2p0_d2p0_v1p0_zvl128b1p0_zvl32b1p0_zvl64b1p0"
	.file	"hello_world.c"
	.globl	factorial
	.p2align	2
	.type	factorial,@function
factorial:
	beqz	a0, .LBB0_2
	addi	sp, sp, -16
	sw	ra, 12(sp)
	sw	s0, 8(sp)
	mv	s0, a0
	addi	a0, a0, -1
	call	factorial
	mul	a0, a0, s0
	lw	ra, 12(sp)
	lw	s0, 8(sp)
	addi	sp, sp, 16
	ret
.LBB0_2:
	li	a0, 1
	ret
.Lfunc_end0:
	.size	factorial, .Lfunc_end0-factorial

	.globl	fac_test
	.p2align	2
	.type	fac_test,@function
fac_test:
	addi	sp, sp, -16
	sw	ra, 12(sp)
	li	a0, 10
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
	li	a0, 1
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
	lw	ra, 12(sp)
	lw	s0, 8(sp)
	lw	s1, 4(sp)
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
	li	a0, 10
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
	li	a1, 0
	addi	a2, sp, 8
	li	a3, 1
	li	a4, 50
	j	.LBB4_3
.LBB4_1:
	lw	a5, -4(a2)
	lw	a6, -8(a2)
	add	a5, a6, a5
	sw	a5, 0(a2)
.LBB4_2:
	addi	a1, a1, 1
	addi	a2, a2, 4
	beq	a1, a4, .LBB4_7
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
	li	a0, 33
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
	lui	a1, %hi(outputAttempted)
	sw	zero, %lo(outputAttempted+4)(a1)
	li	a2, 7
	sw	a2, %lo(outputAttempted)(a1)
	srai	a1, a0, 31
	lui	a2, %hi(outputSucceeded)
	sw	a0, %lo(outputSucceeded)(a2)
	sw	a1, %lo(outputSucceeded+4)(a2)
	lw	ra, 12(sp)
	lw	s0, 8(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end6:
	.size	main, .Lfunc_end6-main

	.ident	"clang version 15.0.0 (https://github.com/llvm/llvm-project.git 853e0aa424e40b80d0bda1dd8a3471a361048e4b)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym outputAttempted
	.addrsig_sym outputSucceeded
