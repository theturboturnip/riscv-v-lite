	.text
	.attribute	4, 16
	.attribute	5, "rv64i2p0_m2p0_v0p10_zvlsseg0p10"
	.file	"hello_world.c"
	.globl	factorial
	.p2align	2
	.type	factorial,@function
factorial:
	addi	sp, sp, -16
	sd	ra, 8(sp)
	sd	s0, 0(sp)
	beqz	a0, .LBB0_2
	mv	s0, a0
	addiw	a0, a0, -1
	call	factorial
	mulw	a0, a0, s0
	j	.LBB0_3
.LBB0_2:
	addi	a0, zero, 1
.LBB0_3:
	ld	s0, 0(sp)
	ld	ra, 8(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end0:
	.size	factorial, .Lfunc_end0-factorial

	.globl	fac_test
	.p2align	2
	.type	fac_test,@function
fac_test:
	addi	sp, sp, -16
	sd	ra, 8(sp)
	addi	a0, zero, 10
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
	addi	a0, zero, 1
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
	ld	s1, 8(sp)
	ld	s0, 16(sp)
	ld	ra, 24(sp)
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
	addi	a0, zero, 10
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
	mv	a1, zero
	mv	a2, zero
	lui	a3, 524288
	addiw	t1, a3, -1
	addi	t0, zero, 1
	addi	a7, zero, 200
	addi	a6, sp, 8
	j	.LBB4_3
.LBB4_1:
	add	a5, a6, a1
	lw	a4, -4(a5)
	lw	a3, -8(a5)
	add	a3, a3, a4
	sw	a3, 0(a5)
.LBB4_2:
	addi	a1, a1, 4
	addi	a2, a2, 1
	beq	a1, a7, .LBB4_7
.LBB4_3:
	and	a5, a2, t1
	beq	a5, t0, .LBB4_6
	bnez	a5, .LBB4_1
	sw	zero, 8(sp)
	j	.LBB4_2
.LBB4_6:
	sw	t0, 12(sp)
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
	addi	a0, zero, 33
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
	slli	a0, a0, 1
	or	s0, a0, s0
	call	fib_memo_test
	slli	a0, a0, 2
	or	a0, s0, a0
	lui	a1, %hi(outputAttempted)
	addi	a2, zero, 7
	sd	a2, %lo(outputAttempted)(a1)
	sext.w	a0, a0
	lui	a1, %hi(outputSucceeded)
	sd	a0, %lo(outputSucceeded)(a1)
	lui	a1, %hi(finished)
	addi	a2, zero, 1
	sb	a2, %lo(finished)(a1)
	ld	s0, 0(sp)
	ld	ra, 8(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end6:
	.size	main, .Lfunc_end6-main

	.ident	"Ubuntu clang version 13.0.1-++20220120110924+75e33f71c2da-1~exp1~20220120231001.58"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym outputAttempted
	.addrsig_sym outputSucceeded
	.addrsig_sym finished
