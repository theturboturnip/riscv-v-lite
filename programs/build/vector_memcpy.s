	.text
	.attribute	4, 16
	.attribute	5, "rv32i2p0_m2p0_a2p0_f2p0_d2p0_v0p10_zvlsseg0p10"
	.file	"vector_memcpy.c"
	.globl	memset
	.p2align	2
	.type	memset,@function
memset:
	beqz	a2, .LBB0_3
	mv	a3, a0
.LBB0_2:
	sb	a1, 0(a3)
	addi	a2, a2, -1
	addi	a3, a3, 1
	bnez	a2, .LBB0_2
.LBB0_3:
	ret
.Lfunc_end0:
	.size	memset, .Lfunc_end0-memset

	.globl	vector_memcpy_strided
	.p2align	2
	.type	vector_memcpy_strided,@function
vector_memcpy_strided:
	beqz	a0, .LBB1_8
	addi	t0, zero, 4
	addi	a7, zero, 16
	j	.LBB1_4
.LBB1_2:
	vle32.v	v25, (a1)
	vse32.v	v25, (a2)
.LBB1_3:
	sub	a0, a0, a5
	slli	a3, a5, 2
	add	a1, a1, a3
	add	a2, a2, a3
	beqz	a0, .LBB1_8
.LBB1_4:
	vsetvli	a5, a0, e32, m1, ta, mu
	slli	a6, a5, 2
	bgeu	a6, a0, .LBB1_2
	mv	a4, zero
.LBB1_6:
	add	a3, a1, a4
	vsetvli	zero, a5, e32, m1, ta, mu
	vlse32.v	v25, (a3), t0
	add	a3, a2, a4
	addi	a4, a4, 4
	vsse32.v	v25, (a3), t0
	bne	a4, a7, .LBB1_6
	mv	a5, a6
	j	.LBB1_3
.LBB1_8:
	ret
.Lfunc_end1:
	.size	vector_memcpy_strided, .Lfunc_end1-vector_memcpy_strided

	.globl	vector_memcpy_32mf2
	.p2align	2
	.type	vector_memcpy_32mf2,@function
vector_memcpy_32mf2:
	beqz	a0, .LBB2_2
.LBB2_1:
	vsetvli	a3, a0, e32, mf2, ta, mu
	vle32.v	v25, (a1)
	vse32.v	v25, (a2)
	slli	a4, a3, 2
	add	a1, a1, a4
	sub	a0, a0, a3
	add	a2, a2, a4
	bnez	a0, .LBB2_1
.LBB2_2:
	ret
.Lfunc_end2:
	.size	vector_memcpy_32mf2, .Lfunc_end2-vector_memcpy_32mf2

	.globl	vector_memcpy_32m8
	.p2align	2
	.type	vector_memcpy_32m8,@function
vector_memcpy_32m8:
	beqz	a0, .LBB3_2
.LBB3_1:
	vsetvli	a3, a0, e32, m8, ta, mu
	vle32.v	v8, (a1)
	vse32.v	v8, (a2)
	slli	a4, a3, 2
	add	a1, a1, a4
	sub	a0, a0, a3
	add	a2, a2, a4
	bnez	a0, .LBB3_1
.LBB3_2:
	ret
.Lfunc_end3:
	.size	vector_memcpy_32m8, .Lfunc_end3-vector_memcpy_32m8

	.globl	vector_memcpy_harness
	.p2align	2
	.type	vector_memcpy_harness,@function
vector_memcpy_harness:
	addi	sp, sp, -1056
	sw	ra, 1052(sp)
	sw	s0, 1048(sp)
	sw	s1, 1044(sp)
	sw	s2, 1040(sp)
	mv	s2, a0
	addi	a0, sp, 528
	addi	a2, zero, 512
	addi	s1, sp, 528
	mv	a1, zero
	call	memset@plt
	addi	a0, sp, 16
	addi	a2, zero, 512
	mv	a1, zero
	call	memset@plt
	mv	a0, zero
	addi	a1, zero, 128
.LBB4_1:
	sw	a0, 0(s1)
	addi	a0, a0, 1
	addi	s1, s1, 4
	bne	a0, a1, .LBB4_1
	addi	s1, sp, 528
	addi	a0, zero, 103
	addi	a1, sp, 528
	addi	a2, sp, 16
	addi	s0, sp, 16
	jalr	s2
	mv	a0, zero
	addi	a1, zero, 412
.LBB4_3:
	add	a2, s1, a0
	lw	a2, 0(a2)
	add	a3, s0, a0
	lw	a3, 0(a3)
	bne	a2, a3, .LBB4_10
	addi	a0, a0, 4
	bne	a0, a1, .LBB4_3
	lw	a1, 428(sp)
	mv	a0, zero
	bnez	a1, .LBB4_11
	addi	a0, sp, 432
	addi	a3, zero, 103
	addi	a1, zero, 127
.LBB4_7:
	mv	a2, a3
	beq	a3, a1, .LBB4_9
	lw	a4, 0(a0)
	addi	a0, a0, 4
	addi	a3, a2, 1
	beqz	a4, .LBB4_7
.LBB4_9:
	addi	a0, zero, 126
	sltu	a0, a0, a2
	j	.LBB4_11
.LBB4_10:
	mv	a0, zero
.LBB4_11:
	lw	s2, 1040(sp)
	lw	s1, 1044(sp)
	lw	s0, 1048(sp)
	lw	ra, 1052(sp)
	addi	sp, sp, 1056
	ret
.Lfunc_end4:
	.size	vector_memcpy_harness, .Lfunc_end4-vector_memcpy_harness

	.globl	main
	.p2align	2
	.type	main,@function
main:
	addi	sp, sp, -16
	sw	ra, 12(sp)
	sw	s0, 8(sp)
	lui	a0, %hi(vector_memcpy_32m8)
	addi	a0, a0, %lo(vector_memcpy_32m8)
	call	vector_memcpy_harness
	mv	s0, a0
	lui	a0, %hi(vector_memcpy_32mf2)
	addi	a0, a0, %lo(vector_memcpy_32mf2)
	call	vector_memcpy_harness
	slli	a0, a0, 1
	or	s0, a0, s0
	lui	a0, %hi(vector_memcpy_strided)
	addi	a0, a0, %lo(vector_memcpy_strided)
	call	vector_memcpy_harness
	slli	a0, a0, 2
	or	a0, s0, a0
	lui	a1, 983040
	sw	a0, 0(a1)
	lw	s0, 8(sp)
	lw	ra, 12(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end5:
	.size	main, .Lfunc_end5-main

	.ident	"Ubuntu clang version 13.0.1-++20211215022811+5932c004778c-1~exp1~20211215022819.27"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym vector_memcpy_strided
	.addrsig_sym vector_memcpy_32mf2
	.addrsig_sym vector_memcpy_32m8
