	.text
	.attribute	4, 16
	.attribute	5, "rv64i2p0_m2p0_f2p0_d2p0_v1p0_zvl128b1p0_zvl32b1p0_zvl64b1p0"
	.file	"hello_world.c"
	.globl	factorial
	.p2align	2
	.type	factorial,@function
factorial:
	beqz	a0, .LBB0_2
	addi	sp, sp, -16
	sd	ra, 8(sp)
	sd	s0, 0(sp)
	mv	s0, a0
	addiw	a0, a0, -1
	call	factorial
	mulw	a0, a0, s0
	ld	ra, 8(sp)
	ld	s0, 0(sp)
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
	sd	ra, 8(sp)
	li	a0, 10
	call	factorial
	lui	a1, 886
	addiw	a1, a1, -256
	xor	a0, a0, a1
	seqz	a0, a0
	ld	ra, 8(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end1:
	.size	fac_test, .Lfunc_end1-fac_test

	.globl	fibbonacci
	.p2align	2
	.type	fibbonacci,@function
fibbonacci:
	addi	sp, sp, -32
	sd	ra, 24(sp)
	sd	s0, 16(sp)
	sd	s1, 8(sp)
	mv	s0, a0
	beqz	a0, .LBB2_3
	li	a0, 1
	beq	s0, a0, .LBB2_4
	addiw	a0, s0, -1
	call	fibbonacci
	mv	s1, a0
	addiw	a0, s0, -2
	call	fibbonacci
	addw	a0, a0, s1
	j	.LBB2_4
.LBB2_3:
	mv	a0, s0
.LBB2_4:
	ld	ra, 24(sp)
	ld	s0, 16(sp)
	ld	s1, 8(sp)
	addi	sp, sp, 32
	ret
.Lfunc_end2:
	.size	fibbonacci, .Lfunc_end2-fibbonacci

	.globl	fib_test
	.p2align	2
	.type	fib_test,@function
fib_test:
	addi	sp, sp, -16
	sd	ra, 8(sp)
	li	a0, 10
	call	fibbonacci
	addi	a0, a0, -55
	seqz	a0, a0
	ld	ra, 8(sp)
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
	li	a2, 0
	li	a3, 1
	li	a4, 200
	addi	a5, sp, 8
	j	.LBB4_3
.LBB4_1:
	add	a6, a5, a1
	lw	a7, -4(a6)
	lw	t0, -8(a6)
	addw	a7, t0, a7
	sw	a7, 0(a6)
.LBB4_2:
	addi	a1, a1, 4
	addiw	a2, a2, 1
	beq	a1, a4, .LBB4_7
.LBB4_3:
	slli	a6, a2, 33
	srli	a6, a6, 33
	beq	a6, a3, .LBB4_6
	bnez	a6, .LBB4_1
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
	sd	ra, 8(sp)
	li	a0, 33
	call	fib_memo
	lui	a1, 860
	addiw	a1, a1, 2018
	xor	a0, a0, a1
	seqz	a0, a0
	ld	ra, 8(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end5:
	.size	fib_memo_test, .Lfunc_end5-fib_memo_test

	.globl	main
	.p2align	2
	.type	main,@function
main:
	addi	sp, sp, -16
	sd	ra, 8(sp)
	sd	s0, 0(sp)
	call	fac_test
	mv	s0, a0
	call	fib_test
	slliw	a0, a0, 1
	or	s0, a0, s0
	call	fib_memo_test
	slliw	a0, a0, 2
	or	a0, s0, a0
	lui	a1, %hi(outputAttempted)
	li	a2, 7
	sd	a2, %lo(outputAttempted)(a1)
	lui	a1, %hi(outputSucceeded)
	sd	a0, %lo(outputSucceeded)(a1)
	ld	ra, 8(sp)
	ld	s0, 0(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end6:
	.size	main, .Lfunc_end6-main

	.ident	"clang version 15.0.0 (https://github.com/llvm/llvm-project.git 853e0aa424e40b80d0bda1dd8a3471a361048e4b)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym outputAttempted
	.addrsig_sym outputSucceeded
