	.text
	.attribute	4, 16
	.attribute	5, "rv64i2p0_m2p0_v0p10_zvlsseg0p10"
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

	.globl	_Z13vector_memcpyPhPKhm
	.p2align	2
	.type	_Z13vector_memcpyPhPKhm,@function
_Z13vector_memcpyPhPKhm:
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
	.size	_Z13vector_memcpyPhPKhm, .Lfunc_end2-_Z13vector_memcpyPhPKhm

	.globl	_Z13run_base_testv
	.p2align	2
	.type	_Z13run_base_testv,@function
_Z13run_base_testv:
	addi	sp, sp, -2032
	sd	ra, 2024(sp)
	sd	s0, 2016(sp)
	lui	a0, 1
	addiw	a0, a0, -1968
	sub	sp, sp, a0
	mv	a0, zero
	lui	a1, %hi(.L__const._Z13run_base_testv.bases)
	ld	a2, %lo(.L__const._Z13run_base_testv.bases)(a1)
	addi	a1, a1, %lo(.L__const._Z13run_base_testv.bases)
	ld	a3, 24(a1)
	ld	a4, 16(a1)
	ld	a1, 8(a1)
	lui	a5, 1
	addiw	a5, a5, 16
	add	a5, sp, a5
	sd	a2, 0(a5)
	lui	a2, 1
	addiw	a2, a2, 40
	add	a2, sp, a2
	sd	a3, 0(a2)
	lui	a2, 1
	addiw	a2, a2, 32
	add	a2, sp, a2
	sd	a4, 0(a2)
	lui	a2, 1
	addiw	a2, a2, 24
	add	a2, sp, a2
	sd	a1, 0(a2)
	lui	a1, 1
	addiw	a1, a1, -2024
	add	a1, sp, a1
	lui	a2, %hi(.L__const._Z13run_base_testv.indices)
	addi	a2, a2, %lo(.L__const._Z13run_base_testv.indices)
	lui	a3, 1
	addiw	a3, a3, 16
	add	a3, sp, a3
	addi	a4, zero, 512
.LBB3_1:
	add	a5, a0, a2
	lw	a5, 0(a5)
	slli	a5, a5, 3
	add	a5, a3, a5
	ld	s0, 0(a5)
	sd	s0, -8(a1)
	sd	a5, 0(a1)
	addi	a0, a0, 4
	addi	a1, a1, 16
	bne	a0, a4, .LBB3_1
	lui	a0, 1
	addiw	s0, a0, -2048
	addi	a0, sp, 16
	mv	a1, zero
	mv	a2, s0
	call	memset@plt
	addi	a0, sp, 16
	lui	a1, 1
	addiw	a1, a1, -2032
	add	a1, sp, a1
	mv	a2, s0
	call	_Z13vector_memcpyPhPKhm
	mv	a0, zero
	addi	a1, sp, 24
	lui	a2, %hi(.L__const._Z13run_base_testv.indices)
	addi	a2, a2, %lo(.L__const._Z13run_base_testv.indices)
	lui	a3, 1
	addiw	a3, a3, 16
	add	a3, sp, a3
	addi	a6, zero, 512
.LBB3_3:
	ld	a5, 0(a1)
	ld	s0, 0(a5)
	ld	a4, -8(a1)
	bne	s0, a4, .LBB3_7
	add	a4, a0, a2
	lw	a4, 0(a4)
	slli	a4, a4, 3
	add	a4, a3, a4
	bne	a5, a4, .LBB3_7
	addi	a0, a0, 4
	addi	a1, a1, 16
	bne	a0, a6, .LBB3_3
	addi	a0, zero, 1
	j	.LBB3_8
.LBB3_7:
	mv	a0, zero
.LBB3_8:
	lui	a1, 1
	addiw	a1, a1, -1968
	add	sp, sp, a1
	ld	s0, 2016(sp)
	ld	ra, 2024(sp)
	addi	sp, sp, 2032
	ret
.Lfunc_end3:
	.size	_Z13run_base_testv, .Lfunc_end3-_Z13run_base_testv

	.globl	main
	.p2align	2
	.type	main,@function
main:
	addi	sp, sp, -16
	sd	ra, 8(sp)
	call	_Z13run_base_testv
	lui	a1, %hi(outputAttempted)
	addi	a2, zero, 1
	sw	a2, %lo(outputAttempted)(a1)
	lui	a1, %hi(outputSucceeded)
	sw	a0, %lo(outputSucceeded)(a1)
	ld	ra, 8(sp)
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

	.ident	"Ubuntu clang version 13.0.1-++20220120110924+75e33f71c2da-1~exp1~20220120231001.58"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym outputAttempted
	.addrsig_sym outputSucceeded
