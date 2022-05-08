	.text
	.attribute	4, 16
	.attribute	5, "rv64i2p0_m2p0_v0p10_zvlsseg0p10_xcheri0p0"
	.file	"vector_memcpy_pointers.cpp"
	.file	1 "/home/samuel/cheri/output/sdk/lib/clang/13.0.0/include" "stddef.h"
	.file	2 "/home/samuel/cheri/output/sdk/lib/clang/13.0.0/include" "stdint.h"
	.file	3 "/media/common/University/Edu/Year4_Masters/Project/riscv-v-lite/programs" "vector_memcpy_pointers/vector_memcpy_pointers.cpp"
	.globl	memset
	.p2align	2
	.type	memset,@function
memset:
.Lfunc_begin0:
	.loc	3 8 0
	.cfi_sections .debug_frame
	.cfi_startproc
	.loc	3 11 5 prologue_end
	beqz	a2, .LBB0_3
.Ltmp0:
	.loc	3 0 5 is_stmt 0
	mv	a3, zero
.Ltmp1:
.LBB0_2:
	.loc	3 12 19 is_stmt 1
	cincoffset	ca4, ca0, a3
.Ltmp2:
	.loc	3 11 33
	addi	a3, a3, 1
.Ltmp3:
	.loc	3 12 24
	csb	a1, 0(ca4)
.Ltmp4:
	.loc	3 11 5
	bne	a2, a3, .LBB0_2
.Ltmp5:
.LBB0_3:
	.loc	3 15 5
	cret
.Ltmp6:
.Lfunc_end0:
	.size	memset, .Lfunc_end0-memset
	.cfi_endproc

	.globl	memcpy
	.p2align	2
	.type	memcpy,@function
memcpy:
.Lfunc_begin1:
	.loc	3 19 0
	.cfi_startproc
	.loc	3 22 5 prologue_end
	beqz	a2, .LBB1_3
.Ltmp7:
	.loc	3 0 5 is_stmt 0
	cmove	ca3, ca0
.Ltmp8:
.LBB1_2:
	.loc	3 23 20 is_stmt 1
	clb	a4, 0(ca1)
	.loc	3 23 18 is_stmt 0
	csb	a4, 0(ca3)
	.loc	3 24 16 is_stmt 1
	cincoffset	ca3, ca3, 1
.Ltmp9:
	.loc	3 26 14
	addi	a2, a2, -1
.Ltmp10:
	.loc	3 25 15
	cincoffset	ca1, ca1, 1
.Ltmp11:
	.loc	3 22 5
	bnez	a2, .LBB1_2
.Ltmp12:
.LBB1_3:
	.loc	3 28 5
	cret
.Ltmp13:
.Lfunc_end1:
	.size	memcpy, .Lfunc_end1-memcpy
	.cfi_endproc

	.globl	_Z13vector_memcpyPhPKhm
	.p2align	2
	.type	_Z13vector_memcpyPhPKhm,@function
_Z13vector_memcpyPhPKhm:
.Lfunc_begin2:
	.loc	3 151 0
	.cfi_startproc
	addi	a3, zero, 16
.Ltmp14:
	.loc	3 154 5 prologue_end
	bltu	a2, a3, .LBB2_3
.Ltmp15:
	.loc	3 0 5 is_stmt 0
	addi	a3, zero, 15
.Ltmp16:
.LBB2_2:
	.loc	3 155 41 is_stmt 1
	srli	a4, a2, 4
.Ltmp17:
	.loc	3 159 9
	#APP
	vsetvli	a4, a4, e128, m8, tu, mu
	#NO_APP
.Ltmp18:
	.loc	3 160 9
	#APP
	vle128.v	v8, (ca1)
	#NO_APP
	.loc	3 161 9
	#APP
	vse128.v	v8, (ca0)
	#NO_APP
	.loc	3 163 45
	slli	a4, a4, 4
.Ltmp19:
	.loc	3 163 13 is_stmt 0
	cincoffset	ca1, ca1, a4
.Ltmp20:
	.loc	3 165 19 is_stmt 1
	sub	a2, a2, a4
.Ltmp21:
	.loc	3 164 13
	cincoffset	ca0, ca0, a4
.Ltmp22:
	.loc	3 154 5
	bltu	a3, a2, .LBB2_2
.Ltmp23:
.LBB2_3:
	.loc	3 170 5
	beqz	a2, .LBB2_5
.Ltmp24:
.LBB2_4:
	.loc	3 171 34
	vsetvli	a3, a2, e8, m8, ta, mu
.Ltmp25:
	.loc	3 175 9
	#APP
	vle8.v	v8, (ca1)
	#NO_APP
.Ltmp26:
	.loc	3 176 9
	#APP
	vse8.v	v8, (ca0)
	#NO_APP
	.loc	3 182 13
	cincoffset	ca1, ca1, a3
.Ltmp27:
	.loc	3 184 19
	sub	a2, a2, a3
.Ltmp28:
	.loc	3 183 13
	cincoffset	ca0, ca0, a3
.Ltmp29:
	.loc	3 170 5
	bnez	a2, .LBB2_4
.Ltmp30:
.LBB2_5:
	.loc	3 186 1
	cret
.Ltmp31:
.Lfunc_end2:
	.size	_Z13vector_memcpyPhPKhm, .Lfunc_end2-_Z13vector_memcpyPhPKhm
	.cfi_endproc

	.globl	_Z13run_base_testv
	.p2align	2
	.type	_Z13run_base_testv,@function
_Z13run_base_testv:
.Lfunc_begin3:
	.loc	3 188 0
	.cfi_startproc
	cincoffset	csp, csp, -2032
	.cfi_def_cfa_offset 2032
	csc	cra, 2016(csp)
	csc	cs0, 2000(csp)
	csc	cs1, 1984(csp)
	.cfi_offset ra, -16
	.cfi_offset s0, -32
	.cfi_offset s1, -48
	lui	a0, 1048574
	addiw	a0, a0, 1904
	cincoffset	csp, csp, a0
	.cfi_def_cfa_offset 8320
	mv	a0, zero
	lui	a1, 2
	addiw	a1, a1, 16
	cincoffset	ca1, csp, a1
	csetbounds	ca1, ca1, 64
.Ltmp32:
	.loc	3 193 16 prologue_end
	csc	cnull, 48(ca1)
	csc	cnull, 32(ca1)
	csc	cnull, 16(ca1)
	csc	cnull, 0(ca1)
	lui	a2, 1863
	addiw	a2, a2, -271
	slli	a2, a2, 15
	addi	a2, a2, -1709
	slli	a2, a2, 13
	addi	a2, a2, -1207
	slli	a2, a2, 12
	addi	a2, a2, 1882
	lui	a3, 2
	addiw	a3, a3, 16
	cincoffset	ca3, csp, a3
	csd	a2, 0(ca3)
	lui	a2, 1033844
	addiw	a2, a2, -253
	slli	a2, a2, 12
	addi	a2, a2, 541
	slli	a2, a2, 13
	addi	a2, a2, 1579
	slli	a2, a2, 12
	addi	a2, a2, 1188
	lui	a3, 2
	addiw	a3, a3, 32
	cincoffset	ca3, csp, a3
	csd	a2, 0(ca3)
	lui	a2, 3301
	addiw	a2, a2, -205
	slli	a2, a2, 13
	addi	a2, a2, 1479
	slli	a2, a2, 14
	addi	a2, a2, 831
	slli	a2, a2, 12
	addi	a2, a2, -802
	lui	a3, 2
	addiw	a3, a3, 48
	cincoffset	ca3, csp, a3
	csd	a2, 0(ca3)
	lui	a2, 1045152
	addiw	a2, a2, -327
	slli	a2, a2, 14
	addi	a2, a2, -453
	slli	a2, a2, 12
	addi	a2, a2, -657
	slli	a2, a2, 13
	addi	a2, a2, 1687
	lui	a3, 2
	addiw	a3, a3, 64
	cincoffset	ca3, csp, a3
	csd	a2, 0(ca3)
	lui	a2, 1
	lui	a3, 1
	addiw	a3, a3, 16
	cincoffset	ca3, csp, a3
	csetbounds	cs1, ca3, a2
	addi	a2, zero, 128
.LBB3_1:
.LBB3_9:
.Ltmp33:
	.loc	3 223 21
	auipcc	ca3, %captab_pcrel_hi(.L__const._Z13run_base_testv.indices)
	clc	ca3, %pcrel_lo(.LBB3_9)(ca3)
	slli	a4, a0, 2
	cincoffset	ca3, ca3, a4
	clw	a3, 0(ca3)
.Ltmp34:
	.loc	3 225 36
	slli	a3, a3, 4
	cincoffset	ca3, ca1, a3
	.loc	3 225 49 is_stmt 0
	cld	a4, 0(ca3)
	.loc	3 224 25 is_stmt 1
	slli	a5, a0, 5
	cincoffset	cs0, cs1, a5
	csd	a4, 0(cs0)
	ori	a4, a5, 16
	cincoffset	ca4, cs1, a4
.Ltmp35:
	.loc	3 222 34
	addi	a0, a0, 1
.Ltmp36:
	.loc	3 224 25
	csc	ca3, 0(ca4)
.Ltmp37:
	.loc	3 222 5
	bne	a0, a2, .LBB3_1
.Ltmp38:
	.loc	3 0 5 is_stmt 0
	lui	a0, 1
	cincoffset	ca1, csp, 16
	csetbounds	cs0, ca1, a0
	.loc	3 230 13 is_stmt 1
	lui	a2, 1
	cmove	ca0, cs0
	mv	a1, zero
	ccall	memset
.Ltmp39:
	.loc	3 236 5
	lui	a2, 1
	cmove	ca0, cs0
	cmove	ca1, cs1
	ccall	_Z13vector_memcpyPhPKhm
.Ltmp40:
	.loc	3 0 5 is_stmt 0
	mv	a0, zero
	lui	a1, 2
	addiw	a1, a1, 16
	cincoffset	ca1, csp, a1
	csetbounds	ca1, ca1, 64
	addi	a2, zero, 128
.Ltmp41:
.LBB3_3:
	.loc	3 242 27 is_stmt 1
	slli	a4, a0, 5
	ori	a3, a4, 16
	cincoffset	ca3, cs0, a3
	clc	ca3, 0(ca3)
	.loc	3 242 37 is_stmt 0
	cld	a5, 0(ca3)
	.loc	3 242 60
	cincoffset	ca4, cs0, a4
	cld	a4, 0(ca4)
	bne	a5, a4, .LBB3_7
.Ltmp42:
.LBB3_10:
	.loc	3 245 84 is_stmt 1
	auipcc	ca4, %captab_pcrel_hi(.L__const._Z13run_base_testv.indices)
	clc	ca4, %pcrel_lo(.LBB3_10)(ca4)
	slli	a5, a0, 2
	cincoffset	ca4, ca4, a5
	clw	a4, 0(ca4)
	.loc	3 245 78 is_stmt 0
	slli	a4, a4, 4
	cincoffset	ca4, ca1, a4
.Ltmp43:
	.loc	3 0 78
	bne	a3, a4, .LBB3_7
.Ltmp44:
	addi	a0, a0, 1
.Ltmp45:
	.loc	3 239 5 is_stmt 1
	bne	a0, a2, .LBB3_3
.Ltmp46:
	.loc	3 0 5 is_stmt 0
	addi	a0, zero, 1
	j	.LBB3_8
.Ltmp47:
.LBB3_7:
	mv	a0, zero
.Ltmp48:
.LBB3_8:
	.loc	3 250 1 is_stmt 1
	lui	a1, 2
	addiw	a1, a1, -1904
	cincoffset	csp, csp, a1
	clc	cs1, 1984(csp)
	clc	cs0, 2000(csp)
	clc	cra, 2016(csp)
	cincoffset	csp, csp, 2032
	cret
.Ltmp49:
.Lfunc_end3:
	.size	_Z13run_base_testv, .Lfunc_end3-_Z13run_base_testv
	.cfi_endproc

	.globl	main
	.p2align	2
	.type	main,@function
main:
.Lfunc_begin4:
	.loc	3 260 0
	.cfi_startproc
	cincoffset	csp, csp, -16
	.cfi_def_cfa_offset 16
.Ltmp50:
	.loc	3 0 0 prologue_end
	csc	cra, 0(csp)
	.cfi_offset ra, -16
.Ltmp51:
	.loc	3 265 15
	ccall	_Z13run_base_testv
.Ltmp52:
.LBB4_1:
	.loc	3 267 25
	auipcc	ca1, %captab_pcrel_hi(outputAttempted)
	clc	ca1, %pcrel_lo(.LBB4_1)(ca1)
	addi	a2, zero, 1
	csw	a2, 0(ca1)
.LBB4_2:
	.loc	3 268 25
	auipcc	ca1, %captab_pcrel_hi(outputSucceeded)
	clc	ca1, %pcrel_lo(.LBB4_2)(ca1)
.Ltmp53:
	csw	a0, 0(ca1)
	.loc	3 269 5
	clc	cra, 0(csp)
	cincoffset	csp, csp, 16
	cret
.Ltmp54:
.Lfunc_end4:
	.size	main, .Lfunc_end4-main
	.cfi_endproc

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

	.file	4 "/home/samuel/cheri/output/sdk/lib/clang/13.0.0/include" "riscv_vector.h"
	.section	.debug_loc,"",@progbits
.Ldebug_loc0:
	.quad	.Lfunc_begin0-.Lfunc_begin0
	.quad	.Ltmp1-.Lfunc_begin0
	.half	3
	.byte	17
	.byte	0
	.byte	159
	.quad	.Ltmp1-.Lfunc_begin0
	.quad	.Ltmp5-.Lfunc_begin0
	.half	1
	.byte	93
	.quad	0
	.quad	0
.Ldebug_loc1:
	.quad	.Lfunc_begin1-.Lfunc_begin0
	.quad	.Ltmp8-.Lfunc_begin0
	.half	1
	.byte	91
	.quad	0
	.quad	0
.Ldebug_loc2:
	.quad	.Lfunc_begin1-.Lfunc_begin0
	.quad	.Ltmp8-.Lfunc_begin0
	.half	1
	.byte	90
	.quad	.Ltmp8-.Lfunc_begin0
	.quad	.Ltmp12-.Lfunc_begin0
	.half	1
	.byte	93
	.quad	0
	.quad	0
.Ldebug_loc3:
	.quad	.Ltmp17-.Lfunc_begin0
	.quad	.Ltmp18-.Lfunc_begin0
	.half	1
	.byte	94
	.quad	0
	.quad	0
.Ldebug_loc4:
	.quad	.Ltmp18-.Lfunc_begin0
	.quad	.Ltmp19-.Lfunc_begin0
	.half	1
	.byte	94
	.quad	0
	.quad	0
.Ldebug_loc5:
	.quad	.Ltmp25-.Lfunc_begin0
	.quad	.Ltmp30-.Lfunc_begin0
	.half	1
	.byte	93
	.quad	0
	.quad	0
.Ldebug_loc6:
	.quad	.Ltmp26-.Lfunc_begin0
	.quad	.Ltmp30-.Lfunc_begin0
	.half	2
	.byte	144
	.byte	104
	.quad	0
	.quad	0
.Ldebug_loc7:
	.quad	.Ltmp36-.Lfunc_begin0
	.quad	.Ltmp38-.Lfunc_begin0
	.half	1
	.byte	90
	.quad	0
	.quad	0
.Ldebug_loc8:
	.quad	.Ltmp39-.Lfunc_begin0
	.quad	.Ltmp42-.Lfunc_begin0
	.half	3
	.byte	114
	.byte	16
	.byte	159
	.quad	.Ltmp44-.Lfunc_begin0
	.quad	.Lfunc_end3-.Lfunc_begin0
	.half	3
	.byte	114
	.byte	16
	.byte	159
	.quad	0
	.quad	0
.Ldebug_loc9:
	.quad	.Ltmp39-.Lfunc_begin0
	.quad	.Ltmp42-.Lfunc_begin0
	.half	4
	.byte	114
	.byte	144
	.byte	32
	.byte	159
	.quad	.Ltmp44-.Lfunc_begin0
	.quad	.Lfunc_end3-.Lfunc_begin0
	.half	4
	.byte	114
	.byte	144
	.byte	32
	.byte	159
	.quad	0
	.quad	0
.Ldebug_loc10:
	.quad	.Ltmp41-.Lfunc_begin0
	.quad	.Ltmp42-.Lfunc_begin0
	.half	1
	.byte	90
	.quad	.Ltmp45-.Lfunc_begin0
	.quad	.Ltmp46-.Lfunc_begin0
	.half	1
	.byte	90
	.quad	0
	.quad	0
.Ldebug_loc11:
	.quad	.Ltmp51-.Lfunc_begin0
	.quad	.Ltmp52-.Lfunc_begin0
	.half	3
	.byte	17
	.byte	0
	.byte	159
	.quad	0
	.quad	0
	.section	.debug_abbrev,"",@progbits
	.byte	1
	.byte	17
	.byte	1
	.byte	37
	.byte	14
	.byte	19
	.byte	5
	.byte	3
	.byte	14
	.byte	16
	.byte	23
	.byte	27
	.byte	14
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	0
	.byte	0
	.byte	2
	.byte	36
	.byte	0
	.byte	3
	.byte	14
	.byte	62
	.byte	11
	.byte	11
	.byte	11
	.byte	0
	.byte	0
	.byte	3
	.byte	15
	.byte	0
	.byte	73
	.byte	19
	.byte	11
	.byte	11
	.byte	0
	.byte	0
	.byte	4
	.byte	38
	.byte	0
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	5
	.byte	22
	.byte	0
	.byte	73
	.byte	19
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	0
	.byte	0
	.byte	6
	.byte	19
	.byte	1
	.byte	54
	.byte	11
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	7
	.byte	13
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	56
	.byte	11
	.byte	0
	.byte	0
	.byte	8
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.ascii	"\227B"
	.byte	25
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	63
	.byte	25
	.byte	0
	.byte	0
	.byte	9
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	10
	.byte	52
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	11
	.byte	11
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	0
	.byte	0
	.byte	12
	.byte	52
	.byte	0
	.byte	2
	.byte	23
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	13
	.byte	5
	.byte	0
	.byte	2
	.byte	23
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	14
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.ascii	"\227B"
	.byte	25
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	63
	.byte	25
	.byte	0
	.byte	0
	.byte	15
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.ascii	"\227B"
	.byte	25
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	63
	.byte	25
	.byte	0
	.byte	0
	.byte	16
	.byte	52
	.byte	0
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	17
	.byte	11
	.byte	1
	.byte	85
	.byte	23
	.byte	0
	.byte	0
	.byte	18
	.ascii	"\211\202\001"
	.byte	0
	.byte	49
	.byte	19
	.byte	17
	.byte	1
	.byte	0
	.byte	0
	.byte	19
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.ascii	"\227B"
	.byte	25
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	63
	.byte	25
	.byte	0
	.byte	0
	.byte	20
	.byte	52
	.byte	0
	.byte	2
	.byte	23
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	21
	.byte	52
	.byte	0
	.byte	28
	.byte	13
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	5
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	22
	.byte	15
	.byte	0
	.byte	11
	.byte	11
	.byte	0
	.byte	0
	.byte	23
	.byte	38
	.byte	0
	.byte	0
	.byte	0
	.byte	24
	.byte	22
	.byte	0
	.byte	73
	.byte	19
	.byte	3
	.byte	14
	.byte	0
	.byte	0
	.byte	25
	.byte	1
	.byte	1
	.ascii	"\207B"
	.byte	25
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	26
	.byte	33
	.byte	0
	.byte	73
	.byte	19
	.byte	34
	.byte	13
	.byte	47
	.byte	24
	.byte	0
	.byte	0
	.byte	27
	.byte	36
	.byte	0
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.byte	62
	.byte	11
	.byte	0
	.byte	0
	.byte	28
	.byte	1
	.byte	1
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	29
	.byte	33
	.byte	0
	.byte	73
	.byte	19
	.byte	55
	.byte	11
	.byte	0
	.byte	0
	.byte	30
	.byte	13
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	56
	.byte	11
	.byte	0
	.byte	0
	.byte	0
	.section	.debug_info,"",@progbits
.Lcu_begin0:
	.word	.Ldebug_info_end0-.Ldebug_info_start0
.Ldebug_info_start0:
	.half	4
	.word	.debug_abbrev
	.byte	8
	.byte	1
	.word	.Linfo_string0
	.half	33
	.word	.Linfo_string1
	.word	.Lline_table_start0
	.word	.Linfo_string2
	.quad	.Lfunc_begin0
	.word	.Lfunc_end4-.Lfunc_begin0
	.byte	2
	.word	.Linfo_string3
	.byte	8
	.byte	1
	.byte	3
	.word	42
	.byte	16
	.byte	3
	.word	61
	.byte	16
	.byte	4
	.word	42
	.byte	5
	.word	77
	.word	.Linfo_string5
	.byte	1
	.byte	50
	.byte	2
	.word	.Linfo_string4
	.byte	7
	.byte	8
	.byte	3
	.word	90
	.byte	16
	.byte	4
	.word	95
	.byte	6
	.byte	5
	.word	.Linfo_string8
	.byte	16
	.byte	3
	.byte	131
	.byte	16
	.byte	7
	.word	.Linfo_string6
	.word	118
	.byte	3
	.byte	132
	.byte	0
	.byte	0
	.byte	5
	.word	77
	.word	.Linfo_string7
	.byte	2
	.byte	98
	.byte	3
	.word	135
	.byte	16
	.byte	5
	.word	42
	.word	.Linfo_string9
	.byte	2
	.byte	226
	.byte	3
	.word	152
	.byte	16
	.byte	4
	.word	135
	.byte	8
	.quad	.Lfunc_begin0
	.word	.Lfunc_end0-.Lfunc_begin0
	.byte	1
	.byte	82

	.word	.Linfo_string10
	.byte	3
	.byte	8
	.word	800

	.byte	9
	.byte	1
	.byte	90
	.word	.Linfo_string18
	.byte	3
	.byte	8
	.word	800
	.byte	9
	.byte	1
	.byte	91
	.word	.Linfo_string19
	.byte	3
	.byte	8
	.word	802
	.byte	9
	.byte	1
	.byte	92
	.word	.Linfo_string20
	.byte	3
	.byte	8
	.word	66
	.byte	10
	.byte	3
	.byte	123
	.byte	0
	.byte	159
	.word	.Linfo_string21
	.byte	3
	.byte	9
	.word	42
	.byte	10
	.byte	1
	.byte	90
	.word	.Linfo_string22
	.byte	3
	.byte	10
	.word	49
	.byte	11
	.quad	.Lfunc_begin0
	.word	.Ltmp5-.Lfunc_begin0
	.byte	12
	.word	.Ldebug_loc0
	.word	.Linfo_string23
	.byte	3
	.byte	11
	.word	802
	.byte	0
	.byte	0
	.byte	8
	.quad	.Lfunc_begin1
	.word	.Lfunc_end1-.Lfunc_begin1
	.byte	1
	.byte	82

	.word	.Linfo_string11
	.byte	3
	.byte	19
	.word	800

	.byte	9
	.byte	1
	.byte	90
	.word	.Linfo_string18
	.byte	3
	.byte	19
	.word	800
	.byte	13
	.word	.Ldebug_loc1
	.word	.Linfo_string24
	.byte	3
	.byte	19
	.word	809
	.byte	9
	.byte	1
	.byte	92
	.word	.Linfo_string20
	.byte	3
	.byte	19
	.word	66
	.byte	12
	.word	.Ldebug_loc2
	.word	.Linfo_string22
	.byte	3
	.byte	20
	.word	49
	.byte	10
	.byte	1
	.byte	91
	.word	.Linfo_string25
	.byte	3
	.byte	21
	.word	55
	.byte	0
	.byte	14
	.quad	.Lfunc_begin2
	.word	.Lfunc_end2-.Lfunc_begin2
	.byte	1
	.byte	82

	.word	.Linfo_string12
	.word	.Linfo_string13
	.byte	3
	.byte	151

	.byte	9
	.byte	1
	.byte	90
	.word	.Linfo_string26
	.byte	3
	.byte	151
	.word	129
	.byte	9
	.byte	1
	.byte	91
	.word	.Linfo_string24
	.byte	3
	.byte	151
	.word	146
	.byte	9
	.byte	1
	.byte	92
	.word	.Linfo_string27
	.byte	3
	.byte	151
	.word	66
	.byte	11
	.quad	.Ltmp16
	.word	.Ltmp22-.Ltmp16
	.byte	12
	.word	.Ldebug_loc3
	.word	.Linfo_string28
	.byte	3
	.byte	155
	.word	66
	.byte	12
	.word	.Ldebug_loc4
	.word	.Linfo_string29
	.byte	3
	.byte	156
	.word	66
	.byte	0
	.byte	11
	.quad	.Ltmp24
	.word	.Ltmp29-.Ltmp24
	.byte	12
	.word	.Ldebug_loc5
	.word	.Linfo_string30
	.byte	3
	.byte	171
	.word	66
	.byte	12
	.word	.Ldebug_loc6
	.word	.Linfo_string31
	.byte	3
	.byte	173
	.word	816
	.byte	0
	.byte	0
	.byte	15
	.quad	.Lfunc_begin3
	.word	.Lfunc_end3-.Lfunc_begin3
	.byte	1
	.byte	82

	.word	.Linfo_string14
	.word	.Linfo_string15
	.byte	3
	.byte	188
	.word	802

	.byte	10
	.byte	4
	.byte	145
	.asciz	"\220\300"
	.word	.Linfo_string35
	.byte	3
	.byte	193
	.word	864
	.byte	10
	.byte	3
	.byte	145
	.ascii	"\220 "
	.word	.Linfo_string36
	.byte	3
	.byte	221
	.word	876
	.byte	10
	.byte	2
	.byte	145
	.byte	16
	.word	.Linfo_string40
	.byte	3
	.byte	230
	.word	876
	.byte	12
	.word	.Ldebug_loc8
	.word	.Linfo_string41
	.byte	3
	.byte	233
	.word	924
	.byte	12
	.word	.Ldebug_loc9
	.word	.Linfo_string42
	.byte	3
	.byte	232
	.word	924
	.byte	16
	.word	.Linfo_string43
	.byte	3
	.byte	202
	.word	930
	.byte	11
	.quad	.Ltmp33
	.word	.Ltmp38-.Ltmp33
	.byte	12
	.word	.Ldebug_loc7
	.word	.Linfo_string23
	.byte	3
	.byte	222
	.word	66
	.byte	17
	.word	.Ldebug_ranges0
	.byte	16
	.word	.Linfo_string44
	.byte	3
	.byte	223
	.word	802
	.byte	0
	.byte	0
	.byte	11
	.quad	.Ltmp41
	.word	.Ltmp46-.Ltmp41
	.byte	12
	.word	.Ldebug_loc10
	.word	.Linfo_string23
	.byte	3
	.byte	239
	.word	66
	.byte	0
	.byte	18
	.word	374
	.quad	.Ltmp40
	.byte	0
	.byte	19
	.quad	.Lfunc_begin4
	.word	.Lfunc_end4-.Lfunc_begin4
	.byte	1
	.byte	82

	.word	.Linfo_string17
	.byte	3
	.half	259
	.word	802

	.byte	20
	.word	.Ldebug_loc11
	.word	.Linfo_string45
	.byte	3
	.half	261
	.word	802
	.byte	21
	.byte	1
	.word	.Linfo_string46
	.byte	3
	.half	262
	.word	802
	.byte	18
	.word	527
	.quad	.Ltmp52
	.byte	0
	.byte	22
	.byte	16
	.byte	2
	.word	.Linfo_string16
	.byte	5
	.byte	4
	.byte	3
	.word	815
	.byte	16
	.byte	23
	.byte	5
	.word	827
	.word	.Linfo_string34
	.byte	4
	.byte	99
	.byte	24
	.word	836
	.word	.Linfo_string33
	.byte	25

	.word	42
	.byte	26
	.word	857
	.byte	0
	.byte	8
	.byte	146
	.ascii	"\2428"
	.byte	0
	.byte	49
	.byte	27
	.byte	56
	.byte	30
	.byte	0
	.byte	27
	.word	.Linfo_string32
	.byte	8
	.byte	7
	.byte	28
	.word	90
	.byte	29
	.word	857
	.byte	4
	.byte	0
	.byte	28
	.word	888
	.byte	29
	.word	857
	.byte	128
	.byte	0
	.byte	6
	.byte	5
	.word	.Linfo_string39
	.byte	32
	.byte	3
	.byte	136
	.byte	16
	.byte	7
	.word	.Linfo_string37
	.word	118
	.byte	3
	.byte	138
	.byte	0
	.byte	30
	.word	.Linfo_string38
	.word	84
	.byte	3
	.byte	140
	.byte	16
	.byte	16
	.byte	0
	.byte	3
	.word	888
	.byte	16
	.byte	28
	.word	802
	.byte	29
	.word	857
	.byte	128
	.byte	0
	.byte	0
.Ldebug_info_end0:
	.section	.debug_ranges,"",@progbits
.Ldebug_ranges0:
	.quad	.Ltmp33-.Lfunc_begin0
	.quad	.Ltmp35-.Lfunc_begin0
	.quad	.Ltmp36-.Lfunc_begin0
	.quad	.Ltmp37-.Lfunc_begin0
	.quad	0
	.quad	0
	.section	.debug_str,"MS",@progbits,1
.Linfo_string0:
	.asciz	"clang version 13.0.0 (ssh://git@github.com/theturboturnip/llvm-project.git 88213dcf1e9bc454f471b9e9a8b2ede325dc5e24)"
.Linfo_string1:
	.asciz	"/media/common/University/Edu/Year4_Masters/Project/riscv-v-lite/programs/vector_memcpy_pointers/vector_memcpy_pointers.cpp"
.Linfo_string2:
	.asciz	"/media/common/University/Edu/Year4_Masters/Project/riscv-v-lite/programs/build/llvm-13-rv64imxcheri/vector_memcpy_pointers"
.Linfo_string3:
	.asciz	"unsigned char"
.Linfo_string4:
	.asciz	"long unsigned int"
.Linfo_string5:
	.asciz	"size_t"
.Linfo_string6:
	.asciz	"value"
.Linfo_string7:
	.asciz	"uint64_t"
.Linfo_string8:
	.asciz	"Base"
.Linfo_string9:
	.asciz	"uint8_t"
.Linfo_string10:
	.asciz	"memset"
.Linfo_string11:
	.asciz	"memcpy"
.Linfo_string12:
	.asciz	"_Z13vector_memcpyPhPKhm"
.Linfo_string13:
	.asciz	"vector_memcpy"
.Linfo_string14:
	.asciz	"_Z13run_base_testv"
.Linfo_string15:
	.asciz	"run_base_test"
.Linfo_string16:
	.asciz	"int"
.Linfo_string17:
	.asciz	"main"
.Linfo_string18:
	.asciz	"dest"
.Linfo_string19:
	.asciz	"ch"
.Linfo_string20:
	.asciz	"count"
.Linfo_string21:
	.asciz	"ch_uc"
.Linfo_string22:
	.asciz	"dest_uc"
.Linfo_string23:
	.asciz	"i"
.Linfo_string24:
	.asciz	"src"
.Linfo_string25:
	.asciz	"src_uc"
.Linfo_string26:
	.asciz	"dst"
.Linfo_string27:
	.asciz	"num_bytes"
.Linfo_string28:
	.asciz	"num_elements"
.Linfo_string29:
	.asciz	"copied_128bit_elems_per_iter"
.Linfo_string30:
	.asciz	"copied_per_iter"
.Linfo_string31:
	.asciz	"data"
.Linfo_string32:
	.asciz	"__ARRAY_SIZE_TYPE__"
.Linfo_string33:
	.asciz	"__rvv_uint8m8_t"
.Linfo_string34:
	.asciz	"vuint8m8_t"
.Linfo_string35:
	.asciz	"bases"
.Linfo_string36:
	.asciz	"source_array"
.Linfo_string37:
	.asciz	"expected_base_value"
.Linfo_string38:
	.asciz	"base_ptr"
.Linfo_string39:
	.asciz	"Element"
.Linfo_string40:
	.asciz	"dest_array"
.Linfo_string41:
	.asciz	"dst_ptr"
.Linfo_string42:
	.asciz	"src_ptr"
.Linfo_string43:
	.asciz	"indices"
.Linfo_string44:
	.asciz	"index"
.Linfo_string45:
	.asciz	"result"
.Linfo_string46:
	.asciz	"attempted"
	.ident	"clang version 13.0.0 (ssh://git@github.com/theturboturnip/llvm-project.git 88213dcf1e9bc454f471b9e9a8b2ede325dc5e24)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym outputAttempted
	.addrsig_sym outputSucceeded
	.section	.debug_line,"",@progbits
.Lline_table_start0:
