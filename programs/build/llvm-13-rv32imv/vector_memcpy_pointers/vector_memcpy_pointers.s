	.text
	.attribute	4, 16
	.attribute	5, "rv32i2p0_m2p0_v0p10_zvlsseg0p10"
	.file	"vector_memcpy_pointers.cpp"
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

	.globl	memcpy
	.p2align	2
	.type	memcpy,@function
memcpy:
	beqz	a2, .LBB1_3
	mv	a3, a0
.LBB1_2:
	lb	a4, 0(a1)
	sb	a4, 0(a3)
	addi	a3, a3, 1
	addi	a2, a2, -1
	addi	a1, a1, 1
	bnez	a2, .LBB1_2
.LBB1_3:
	ret
.Lfunc_end1:
	.size	memcpy, .Lfunc_end1-memcpy

	.globl	_Z13vector_memcpyPhPKhj
	.p2align	2
	.type	_Z13vector_memcpyPhPKhj,@function
_Z13vector_memcpyPhPKhj:
	beqz	a2, .LBB2_2
.LBB2_1:
	vsetvli	a3, a2, e8, m8, ta, mu
	vle8.v	v8, (a1)
	vse8.v	v8, (a0)
	add	a1, a1, a3
	sub	a2, a2, a3
	add	a0, a0, a3
	bnez	a2, .LBB2_1
.LBB2_2:
	ret
.Lfunc_end2:
	.size	_Z13vector_memcpyPhPKhj, .Lfunc_end2-_Z13vector_memcpyPhPKhj

	.globl	_Z13run_base_testv
	.p2align	2
	.type	_Z13run_base_testv,@function
_Z13run_base_testv:
	addi	sp, sp, -2032
	sw	ra, 2028(sp)
	sw	s0, 2024(sp)
	lui	a0, 1
	addi	a0, a0, -1984
	sub	sp, sp, a0
	mv	a0, zero
	lui	a1, %hi(.L__const._Z13run_base_testv.bases)
	lw	a2, %lo(.L__const._Z13run_base_testv.bases+4)(a1)
	lw	a3, %lo(.L__const._Z13run_base_testv.bases)(a1)
	addi	a1, a1, %lo(.L__const._Z13run_base_testv.bases)
	lw	a4, 28(a1)
	lw	a5, 24(a1)
	lui	a6, 1
	addi	a6, a6, 12
	add	a6, sp, a6
	sw	a2, 0(a6)
	lui	a2, 1
	addi	a2, a2, 8
	add	a2, sp, a2
	sw	a3, 0(a2)
	lui	a2, 1
	addi	a2, a2, 36
	add	a2, sp, a2
	sw	a4, 0(a2)
	lui	a2, 1
	addi	a2, a2, 32
	add	a2, sp, a2
	sw	a5, 0(a2)
	lw	a2, 20(a1)
	lw	a3, 16(a1)
	lw	a4, 12(a1)
	lw	a1, 8(a1)
	lui	a5, 1
	addi	a5, a5, 28
	add	a5, sp, a5
	sw	a2, 0(a5)
	lui	a2, 1
	addi	a2, a2, 24
	add	a2, sp, a2
	sw	a3, 0(a2)
	lui	a2, 1
	addi	a2, a2, 20
	add	a2, sp, a2
	sw	a4, 0(a2)
	lui	a2, 1
	addi	a2, a2, 16
	add	a2, sp, a2
	sw	a1, 0(a2)
	lui	a1, 1
	addi	a1, a1, -2032
	add	a1, sp, a1
	lui	a2, %hi(.L__const._Z13run_base_testv.indices)
	addi	a2, a2, %lo(.L__const._Z13run_base_testv.indices)
	lui	a3, 1
	addi	a3, a3, 8
	add	a6, sp, a3
	addi	a4, zero, 512
.LBB3_1:
	add	a5, a0, a2
	lw	a5, 0(a5)
	slli	a5, a5, 3
	add	a5, a6, a5
	ori	s0, a5, 4
	lw	a3, 0(a5)
	lw	s0, 0(s0)
	sw	a3, -8(a1)
	sw	s0, -4(a1)
	sw	a5, 0(a1)
	addi	a0, a0, 4
	addi	a1, a1, 16
	bne	a0, a4, .LBB3_1
	lui	a0, 1
	addi	s0, a0, -2048
	addi	a0, sp, 8
	mv	a1, zero
	mv	a2, s0
	call	memset@plt
	addi	a0, sp, 8
	lui	a1, 1
	addi	a1, a1, -2040
	add	a1, sp, a1
	mv	a2, s0
	call	_Z13vector_memcpyPhPKhj
	mv	a0, zero
	addi	a1, sp, 16
	lui	a2, %hi(.L__const._Z13run_base_testv.indices)
	addi	t0, a2, %lo(.L__const._Z13run_base_testv.indices)
	lui	a2, 1
	addi	a2, a2, 8
	add	a7, sp, a2
	addi	a6, zero, 512
.LBB3_3:
	lw	a5, 0(a1)
	lw	s0, 0(a5)
	lw	a4, 4(a5)
	lw	a3, -4(a1)
	lw	a2, -8(a1)
	xor	a3, a4, a3
	xor	a2, s0, a2
	or	a2, a2, a3
	bnez	a2, .LBB3_7
	add	a2, a0, t0
	lw	a2, 0(a2)
	slli	a2, a2, 3
	add	a2, a7, a2
	bne	a5, a2, .LBB3_7
	addi	a0, a0, 4
	addi	a1, a1, 16
	bne	a0, a6, .LBB3_3
	addi	a0, zero, 1
	j	.LBB3_8
.LBB3_7:
	mv	a0, zero
.LBB3_8:
	lui	a1, 1
	addi	a1, a1, -1984
	add	sp, sp, a1
	lw	s0, 2024(sp)
	lw	ra, 2028(sp)
	addi	sp, sp, 2032
	ret
.Lfunc_end3:
	.size	_Z13run_base_testv, .Lfunc_end3-_Z13run_base_testv

	.globl	main
	.p2align	2
	.type	main,@function
main:
	addi	sp, sp, -16
	sw	ra, 12(sp)
	call	_Z13run_base_testv
	lui	a1, %hi(outputAttempted)
	addi	a2, zero, 1
	sw	a2, %lo(outputAttempted)(a1)
	lui	a1, %hi(outputSucceeded)
	sw	a0, %lo(outputSucceeded)(a1)
	lw	ra, 12(sp)
	addi	sp, sp, 16
	ret
.Lfunc_end4:
	.size	main, .Lfunc_end4-main

	.type	.L__const._Z13run_base_testv.bases,@object
	.section	.rodata.cst32,"aM",@progbits,32
	.p2align	3
.L__const._Z13run_base_testv.bases:
	.quad	8389908080790640474
	.quad	-8293413467698580316
	.quad	7433078584290569438
	.quad	-7710342347414968681
	.size	.L__const._Z13run_base_testv.bases, 32

	.type	.L__const._Z13run_base_testv.indices,@object
	.section	.rodata,"a",@progbits
	.p2align	2
.L__const._Z13run_base_testv.indices:
	.word	1
	.word	1
	.word	1
	.word	1
	.word	2
	.word	2
	.word	3
	.word	0
	.word	1
	.word	0
	.word	3
	.word	0
	.word	0
	.word	3
	.word	3
	.word	0
	.word	1
	.word	0
	.word	0
	.word	2
	.word	2
	.word	0
	.word	2
	.word	1
	.word	0
	.word	0
	.word	0
	.word	3
	.word	2
	.word	0
	.word	1
	.word	1
	.word	3
	.word	2
	.word	3
	.word	0
	.word	2
	.word	2
	.word	0
	.word	0
	.word	0
	.word	1
	.word	1
	.word	3
	.word	0
	.word	0
	.word	1
	.word	3
	.word	1
	.word	2
	.word	3
	.word	2
	.word	2
	.word	0
	.word	2
	.word	1
	.word	0
	.word	3
	.word	1
	.word	1
	.word	3
	.word	3
	.word	2
	.word	2
	.word	0
	.word	1
	.word	3
	.word	2
	.word	2
	.word	1
	.word	1
	.word	3
	.word	2
	.word	2
	.word	0
	.word	1
	.word	1
	.word	3
	.word	0
	.word	1
	.word	0
	.word	0
	.word	3
	.word	2
	.word	2
	.word	3
	.word	3
	.word	1
	.word	1
	.word	1
	.word	1
	.word	2
	.word	1
	.word	1
	.word	2
	.word	1
	.word	2
	.word	2
	.word	1
	.word	1
	.word	3
	.word	1
	.word	1
	.word	3
	.word	0
	.word	2
	.word	3
	.word	1
	.word	1
	.word	3
	.word	2
	.word	3
	.word	2
	.word	1
	.word	2
	.word	0
	.word	2
	.word	2
	.word	2
	.word	3
	.word	0
	.word	3
	.word	1
	.word	0
	.word	3
	.word	2
	.word	1
	.word	0
	.size	.L__const._Z13run_base_testv.indices, 512

	.ident	"Ubuntu clang version 13.0.1-++20220120110844+75e33f71c2da-1~exp1~20220120230854.66"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym outputAttempted
	.addrsig_sym outputSucceeded
