	.file	"hello_world.c"
	.option nopic
	.attribute arch, "rv64i2p0_m2p0_v1p0_zvamo1p0_zvlsseg1p0"
	.attribute unaligned_access, 0
	.attribute stack_align, 16
	.text
	.align	2
	.globl	factorial
	.type	factorial, @function
factorial:
	bne	a0,zero,.L8
	li	a0,1
	ret
.L8:
	addi	sp,sp,-16
	sd	ra,8(sp)
	sd	s0,0(sp)
	mv	s0,a0
	addiw	a0,a0,-1
	call	factorial
	mulw	a0,a0,s0
	ld	ra,8(sp)
	ld	s0,0(sp)
	addi	sp,sp,16
	jr	ra
	.size	factorial, .-factorial
	.align	2
	.globl	fac_test
	.type	fac_test, @function
fac_test:
	addi	sp,sp,-16
	sd	ra,8(sp)
	li	a0,10
	call	factorial
	li	a5,3629056
	addi	a5,a5,-256
	sub	a0,a0,a5
	seqz	a0,a0
	ld	ra,8(sp)
	addi	sp,sp,16
	jr	ra
	.size	fac_test, .-fac_test
	.align	2
	.globl	fibbonacci
	.type	fibbonacci, @function
fibbonacci:
	addi	sp,sp,-32
	sd	ra,24(sp)
	sd	s0,16(sp)
	sd	s1,8(sp)
	mv	s0,a0
	beq	a0,zero,.L12
	li	a5,1
	bne	a0,a5,.L14
.L12:
	mv	a0,s0
	ld	ra,24(sp)
	ld	s0,16(sp)
	ld	s1,8(sp)
	addi	sp,sp,32
	jr	ra
.L14:
	addiw	a0,a0,-1
	call	fibbonacci
	mv	s1,a0
	addiw	a0,s0,-2
	call	fibbonacci
	addw	s0,s1,a0
	j	.L12
	.size	fibbonacci, .-fibbonacci
	.align	2
	.globl	fib_test
	.type	fib_test, @function
fib_test:
	addi	sp,sp,-16
	sd	ra,8(sp)
	li	a0,10
	call	fibbonacci
	addi	a0,a0,-55
	seqz	a0,a0
	ld	ra,8(sp)
	addi	sp,sp,16
	jr	ra
	.size	fib_test, .-fib_test
	.align	2
	.globl	fib_memo
	.type	fib_memo, @function
fib_memo:
	addi	sp,sp,-208
	addi	a5,sp,8
	li	a2,1
	li	a4,0
	li	a6,1
	li	a7,49
	j	.L22
.L25:
	sw	zero,8(sp)
.L23:
	addiw	a4,a4,1
	addiw	a2,a2,1
	addi	a5,a5,4
.L22:
	sext.w	a3,a4
	beq	a3,zero,.L25
	beq	a3,a6,.L26
	lw	a3,-4(a5)
	lw	a1,-8(a5)
	addw	a3,a3,a1
	sw	a3,0(a5)
	sext.w	a3,a2
	ble	a3,a7,.L23
	slli	a5,a0,2
	addi	a4,sp,208
	add	a5,a4,a5
	lw	a0,-200(a5)
	addi	sp,sp,208
	jr	ra
.L26:
	sw	a6,12(sp)
	j	.L23
	.size	fib_memo, .-fib_memo
	.align	2
	.globl	fib_memo_test
	.type	fib_memo_test, @function
fib_memo_test:
	addi	sp,sp,-16
	sd	ra,8(sp)
	li	a0,33
	call	fib_memo
	li	a5,3522560
	addi	a5,a5,2018
	sub	a0,a0,a5
	seqz	a0,a0
	ld	ra,8(sp)
	addi	sp,sp,16
	jr	ra
	.size	fib_memo_test, .-fib_memo_test
	.align	2
	.globl	main
	.type	main, @function
main:
	addi	sp,sp,-32
	sd	ra,24(sp)
	sd	s0,16(sp)
	sd	s1,8(sp)
	call	fac_test
	mv	s0,a0
	call	fib_test
	mv	s1,a0
	call	fib_memo_test
	slliw	s1,s1,1
	slliw	a0,a0,2
	or	a0,s1,a0
	or	a0,s0,a0
	sext.w	a0,a0
	lui	a5,%hi(outputAttempted)
	li	a4,7
	sd	a4,%lo(outputAttempted)(a5)
	lui	a5,%hi(outputSucceeded)
	sd	a0,%lo(outputSucceeded)(a5)
	ld	ra,24(sp)
	ld	s0,16(sp)
	ld	s1,8(sp)
	addi	sp,sp,32
	jr	ra
	.size	main, .-main
	.ident	"GCC: (GNU) 10.1.0"
