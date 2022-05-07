	.text
	.attribute	4, 16
	.attribute	5, "rv64i2p0_m2p0_v0p10_zvlsseg0p10_xcheri0p0"
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
	addi	a3, zero, 128
	bltu	a2, a3, .LBB2_3
	addi	a3, zero, 127
.LBB2_2:
	srli	a4, a2, 7
	#APP
	vsetvli	a4, a4, e128, m8, tu, mu
	#NO_APP
	#APP
	vle128.v	v8, (a1)
	#NO_APP
	#APP
	vse128.v	v8, (a0)
	#NO_APP
	slli	a4, a4, 7
	add	a1, a1, a4
	sub	a2, a2, a4
	add	a0, a0, a4
	bltu	a3, a2, .LBB2_2
.LBB2_3:
	beqz	a2, .LBB2_5
.LBB2_4:
	vsetvli	a3, a2, e8, m8, ta, mu
	#APP
	vle8.v	v8, (a1)
	#NO_APP
	#APP
	vse8.v	v8, (a0)
	#NO_APP
	add	a1, a1, a3
	sub	a2, a2, a3
	add	a0, a0, a3
	bnez	a2, .LBB2_4
.LBB2_5:
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
	lui	a0, 2
	addiw	a0, a0, -1968
	sub	sp, sp, a0
	lui	a0, %hi(.L__const._Z13run_base_testv.bases)
	addi	a1, a0, %lo(.L__const._Z13run_base_testv.bases)
	lui	a0, 2
	addiw	a0, a0, 16
	add	a0, sp, a0
	addi	a2, zero, 32
	lui	a3, 2
	addiw	a3, a3, 16
	add	s0, sp, a3
	call	memcpy@plt
	mv	a0, zero
	lui	a1, 1
	addiw	a1, a1, 32
	add	a1, sp, a1
	lui	a2, %hi(.L__const._Z13run_base_testv.indices)
	addi	a2, a2, %lo(.L__const._Z13run_base_testv.indices)
	addi	a3, zero, 512
.LBB3_1:
	add	a4, a0, a2
	lw	a4, 0(a4)
	slli	a4, a4, 3
	add	a4, s0, a4
	ld	a5, 0(a4)
	cfromptr	ca4, ddc, a4
	sd	a5, -16(a1)
	sc	ca4, 0(a1)
	addi	a0, a0, 4
	addi	a1, a1, 32
	bne	a0, a3, .LBB3_1
	addi	a0, sp, 16
	lui	a2, 1
	mv	a1, zero
	call	memset@plt
	addi	a0, sp, 16
	lui	a1, 1
	addiw	a1, a1, 16
	add	a1, sp, a1
	lui	a2, 1
	call	_Z13vector_memcpyPhPKhm
	mv	a0, zero
	addi	a1, sp, 32
	lui	a2, %hi(.L__const._Z13run_base_testv.indices)
	addi	a2, a2, %lo(.L__const._Z13run_base_testv.indices)
	lui	a3, 2
	addiw	a3, a3, 16
	add	a3, sp, a3
	addi	a6, zero, 512
.LBB3_3:
	lc	ca5, 0(a1)
	ld.cap	s0, (ca5)
	ld	a4, -16(a1)
	bne	s0, a4, .LBB3_7
	add	a4, a0, a2
	lw	a4, 0(a4)
	slli	a4, a4, 3
	add	a4, a3, a4
	cfromptr	ca4, ddc, a4
	bne	a5, a4, .LBB3_7
	addi	a0, a0, 4
	addi	a1, a1, 32
	bne	a0, a6, .LBB3_3
	addi	a0, zero, 1
	j	.LBB3_8
.LBB3_7:
	mv	a0, zero
.LBB3_8:
	lui	a1, 2
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

	.ident	"clang version 13.0.0 (ssh://git@github.com/theturboturnip/llvm-project 301572a8ceabc9f61065cd0a63c32f0ea3319656)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym outputAttempted
	.addrsig_sym outputSucceeded
