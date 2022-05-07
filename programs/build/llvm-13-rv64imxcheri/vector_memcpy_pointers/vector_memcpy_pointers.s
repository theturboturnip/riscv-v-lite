	.text
	.attribute	4, 16
	.attribute	5, "rv64i2p0_m2p0_v0p10_zvlsseg0p10_xcheri0p0"
	.file	"vector_memcpy_pointers.cpp"
	.globl	memset
	.p2align	2
	.type	memset,@function
memset:
	beqz	a2, .LBB0_3
	mv	a3, zero
.LBB0_2:
	cincoffset	ca4, ca0, a3
	addi	a3, a3, 1
	csb	a1, 0(ca4)
	bne	a2, a3, .LBB0_2
.LBB0_3:
	cret
.Lfunc_end0:
	.size	memset, .Lfunc_end0-memset

	.globl	memcpy
	.p2align	2
	.type	memcpy,@function
memcpy:
	beqz	a2, .LBB1_3
	cmove	ca3, ca0
.LBB1_2:
	clb	a4, 0(ca1)
	csb	a4, 0(ca3)
	cincoffset	ca3, ca3, 1
	addi	a2, a2, -1
	cincoffset	ca1, ca1, 1
	bnez	a2, .LBB1_2
.LBB1_3:
	cret
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
	vle128.v	v8, (ca1)
	#NO_APP
	#APP
	vse128.v	v8, (ca0)
	#NO_APP
	slli	a4, a4, 7
	cincoffset	ca1, ca1, a4
	sub	a2, a2, a4
	cincoffset	ca0, ca0, a4
	bltu	a3, a2, .LBB2_2
.LBB2_3:
	beqz	a2, .LBB2_5
.LBB2_4:
	vsetvli	a3, a2, e8, m8, ta, mu
	#APP
	vle8.v	v8, (ca1)
	#NO_APP
	#APP
	vse8.v	v8, (ca0)
	#NO_APP
	cincoffset	ca1, ca1, a3
	sub	a2, a2, a3
	cincoffset	ca0, ca0, a3
	bnez	a2, .LBB2_4
.LBB2_5:
	cret
.Lfunc_end2:
	.size	_Z13vector_memcpyPhPKhm, .Lfunc_end2-_Z13vector_memcpyPhPKhm

	.globl	_Z13run_base_testv
	.p2align	2
	.type	_Z13run_base_testv,@function
_Z13run_base_testv:
	cincoffset	csp, csp, -2032
	csc	cra, 2016(csp)
	csc	cs0, 2000(csp)
	csc	cs1, 1984(csp)
	lui	a0, 1048574
	addiw	a0, a0, 1936
	cincoffset	csp, csp, a0
	lui	a0, 2
	addiw	a0, a0, 16
	cincoffset	ca0, csp, a0
	csetbounds	cs0, ca0, 32
.LBB3_9:
	auipcc	ca1, %captab_pcrel_hi(.L__const._Z13run_base_testv.bases)
	clc	ca1, %pcrel_lo(.LBB3_9)(ca1)
	addi	a2, zero, 32
	cmove	ca0, cs0
	ccall	memcpy
	mv	a0, zero
	lui	a1, 1
	lui	a2, 1
	addiw	a2, a2, 16
	cincoffset	ca2, csp, a2
	csetbounds	cs1, ca2, a1
	addi	a1, zero, 128
.LBB3_1:
.LBB3_10:
	auipcc	ca2, %captab_pcrel_hi(.L__const._Z13run_base_testv.indices)
	clc	ca2, %pcrel_lo(.LBB3_10)(ca2)
	slli	a3, a0, 2
	cincoffset	ca2, ca2, a3
	clw	a2, 0(ca2)
	slli	a2, a2, 3
	cincoffset	ca2, cs0, a2
	cld	a3, 0(ca2)
	slli	a4, a0, 5
	cincoffset	ca5, cs1, a4
	csd	a3, 0(ca5)
	ori	a3, a4, 16
	cincoffset	ca3, cs1, a3
	addi	a0, a0, 1
	csc	ca2, 0(ca3)
	bne	a0, a1, .LBB3_1
	lui	a0, 1
	cincoffset	ca1, csp, 16
	csetbounds	cs0, ca1, a0
	lui	a2, 1
	cmove	ca0, cs0
	mv	a1, zero
	ccall	memset
	lui	a2, 1
	cmove	ca0, cs0
	cmove	ca1, cs1
	ccall	_Z13vector_memcpyPhPKhm
	mv	a0, zero
	lui	a1, 2
	addiw	a1, a1, 16
	cincoffset	ca1, csp, a1
	csetbounds	ca1, ca1, 32
	addi	a2, zero, 128
.LBB3_3:
	slli	a4, a0, 5
	ori	a3, a4, 16
	cincoffset	ca3, cs0, a3
	clc	ca3, 0(ca3)
	cld	a5, 0(ca3)
	cincoffset	ca4, cs0, a4
	cld	a4, 0(ca4)
	bne	a5, a4, .LBB3_7
.LBB3_11:
	auipcc	ca4, %captab_pcrel_hi(.L__const._Z13run_base_testv.indices)
	clc	ca4, %pcrel_lo(.LBB3_11)(ca4)
	slli	a5, a0, 2
	cincoffset	ca4, ca4, a5
	clw	a4, 0(ca4)
	slli	a4, a4, 3
	cincoffset	ca4, ca1, a4
	bne	a3, a4, .LBB3_7
	addi	a0, a0, 1
	bne	a0, a2, .LBB3_3
	addi	a0, zero, 1
	j	.LBB3_8
.LBB3_7:
	mv	a0, zero
.LBB3_8:
	lui	a1, 2
	addiw	a1, a1, -1936
	cincoffset	csp, csp, a1
	clc	cs1, 1984(csp)
	clc	cs0, 2000(csp)
	clc	cra, 2016(csp)
	cincoffset	csp, csp, 2032
	cret
.Lfunc_end3:
	.size	_Z13run_base_testv, .Lfunc_end3-_Z13run_base_testv

	.globl	main
	.p2align	2
	.type	main,@function
main:
	cincoffset	csp, csp, -16
	csc	cra, 0(csp)
	ccall	_Z13run_base_testv
.LBB4_1:
	auipcc	ca1, %captab_pcrel_hi(outputAttempted)
	clc	ca1, %pcrel_lo(.LBB4_1)(ca1)
	addi	a2, zero, 1
	csw	a2, 0(ca1)
.LBB4_2:
	auipcc	ca1, %captab_pcrel_hi(outputSucceeded)
	clc	ca1, %pcrel_lo(.LBB4_2)(ca1)
	csw	a0, 0(ca1)
	clc	cra, 0(csp)
	cincoffset	csp, csp, 16
	cret
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
