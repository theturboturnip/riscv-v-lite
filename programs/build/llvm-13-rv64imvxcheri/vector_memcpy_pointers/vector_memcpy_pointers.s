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
	.loc	3 153 0
	.cfi_startproc
	addi	a3, zero, 16
.Ltmp14:
	.loc	3 156 5 prologue_end
	bltu	a2, a3, .LBB2_3
.Ltmp15:
	.loc	3 0 5 is_stmt 0
	addi	a3, zero, 15
.Ltmp16:
.LBB2_2:
	.loc	3 157 41 is_stmt 1
	srli	a4, a2, 4
.Ltmp17:
	.loc	3 161 9
	#APP
	vsetvli	a4, a4, e128, m8, tu, mu
	#NO_APP
.Ltmp18:
	.loc	3 162 9
	#APP
	vle128.v	v8, (ca1)
	#NO_APP
	.loc	3 163 9
	#APP
	vse128.v	v8, (ca0)
	#NO_APP
	.loc	3 165 45
	slli	a4, a4, 4
.Ltmp19:
	.loc	3 165 13 is_stmt 0
	cincoffset	ca1, ca1, a4
.Ltmp20:
	.loc	3 167 19 is_stmt 1
	sub	a2, a2, a4
.Ltmp21:
	.loc	3 166 13
	cincoffset	ca0, ca0, a4
.Ltmp22:
	.loc	3 156 5
	bltu	a3, a2, .LBB2_2
.Ltmp23:
.LBB2_3:
	.loc	3 172 5
	beqz	a2, .LBB2_5
.Ltmp24:
.LBB2_4:
	.loc	3 173 34
	vsetvli	a3, a2, e8, m8, ta, mu
.Ltmp25:
	.loc	3 177 9
	#APP
	vle8.v	v8, (ca1)
	#NO_APP
.Ltmp26:
	.loc	3 178 9
	#APP
	vse8.v	v8, (ca0)
	#NO_APP
	.loc	3 184 13
	cincoffset	ca1, ca1, a3
.Ltmp27:
	.loc	3 186 19
	sub	a2, a2, a3
.Ltmp28:
	.loc	3 185 13
	cincoffset	ca0, ca0, a3
.Ltmp29:
	.loc	3 172 5
	bnez	a2, .LBB2_4
.Ltmp30:
.LBB2_5:
	.loc	3 188 1
	cret
.Ltmp31:
.Lfunc_end2:
	.size	_Z13vector_memcpyPhPKhm, .Lfunc_end2-_Z13vector_memcpyPhPKhm
	.cfi_endproc

	.globl	_Z24vector_memcpy_invalidatePhPKhm
	.p2align	2
	.type	_Z24vector_memcpy_invalidatePhPKhm,@function
_Z24vector_memcpy_invalidatePhPKhm:
.Lfunc_begin3:
	.loc	3 193 0
	.cfi_startproc
	addi	a3, zero, 16
.Ltmp32:
	.loc	3 195 5 prologue_end
	bltu	a2, a3, .LBB3_3
.Ltmp33:
	.loc	3 0 5 is_stmt 0
	addi	a3, zero, 15
.Ltmp34:
.LBB3_2:
	.loc	3 196 41 is_stmt 1
	srli	a4, a2, 4
.Ltmp35:
	.loc	3 201 9
	#APP
	vsetvli	a4, a4, e128, m4, tu, mu
	#NO_APP
.Ltmp36:
	.loc	3 202 9
	#APP
	vle128.v	v8, (ca1)
	#NO_APP
	.loc	3 208 39
	slli	a5, a4, 1
.Ltmp37:
	.loc	3 209 9
	#APP
	vsetvli	zero, a5, e64, m8, tu, mu
	#NO_APP
	.loc	3 211 9
	#APP
	vadd.vi	v8, v8, 0
	#NO_APP
	.loc	3 215 9
	#APP
	vsetvli	zero, zero, e128, m4, tu, mu
	#NO_APP
	.loc	3 216 9
	#APP
	vse128.v	v8, (ca0)
	#NO_APP
	.loc	3 218 45
	slli	a4, a4, 4
.Ltmp38:
	.loc	3 218 13 is_stmt 0
	cincoffset	ca1, ca1, a4
.Ltmp39:
	.loc	3 220 19 is_stmt 1
	sub	a2, a2, a4
.Ltmp40:
	.loc	3 219 13
	cincoffset	ca0, ca0, a4
.Ltmp41:
	.loc	3 195 5
	bltu	a3, a2, .LBB3_2
.Ltmp42:
.LBB3_3:
	.loc	3 224 5
	beqz	a2, .LBB3_5
.Ltmp43:
.LBB3_4:
	.loc	3 225 34
	vsetvli	a3, a2, e8, m8, ta, mu
.Ltmp44:
	.loc	3 229 9
	#APP
	vle8.v	v8, (ca1)
	#NO_APP
.Ltmp45:
	.loc	3 230 9
	#APP
	vse8.v	v8, (ca0)
	#NO_APP
	.loc	3 236 13
	cincoffset	ca1, ca1, a3
.Ltmp46:
	.loc	3 238 19
	sub	a2, a2, a3
.Ltmp47:
	.loc	3 237 13
	cincoffset	ca0, ca0, a3
.Ltmp48:
	.loc	3 224 5
	bnez	a2, .LBB3_4
.Ltmp49:
.LBB3_5:
	.loc	3 240 1
	cret
.Ltmp50:
.Lfunc_end3:
	.size	_Z24vector_memcpy_invalidatePhPKhm, .Lfunc_end3-_Z24vector_memcpy_invalidatePhPKhm
	.cfi_endproc

	.globl	_Z13run_base_testv
	.p2align	2
	.type	_Z13run_base_testv,@function
_Z13run_base_testv:
.Lfunc_begin4:
	.loc	3 243 0
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
.Ltmp51:
	.loc	3 247 10 prologue_end
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
.LBB4_1:
.LBB4_9:
.Ltmp52:
	.loc	3 277 21
	auipcc	ca3, %captab_pcrel_hi(.L__const._Z13run_base_testv.indices)
	clc	ca3, %pcrel_lo(.LBB4_9)(ca3)
	slli	a4, a0, 2
	cincoffset	ca3, ca3, a4
	clw	a3, 0(ca3)
.Ltmp53:
	.loc	3 279 36
	slli	a3, a3, 4
	cincoffset	ca3, ca1, a3
	.loc	3 279 49 is_stmt 0
	cld	a4, 0(ca3)
	.loc	3 278 25 is_stmt 1
	slli	a5, a0, 5
	cincoffset	cs0, cs1, a5
	csd	a4, 0(cs0)
	ori	a4, a5, 16
	cincoffset	ca4, cs1, a4
.Ltmp54:
	.loc	3 276 34
	addi	a0, a0, 1
.Ltmp55:
	.loc	3 278 25
	csc	ca3, 0(ca4)
.Ltmp56:
	.loc	3 276 5
	bne	a0, a2, .LBB4_1
.Ltmp57:
	.loc	3 0 5 is_stmt 0
	lui	a0, 1
	cincoffset	ca1, csp, 16
	csetbounds	cs0, ca1, a0
	.loc	3 284 13 is_stmt 1
	lui	a2, 1
	cmove	ca0, cs0
	mv	a1, zero
	ccall	memset
.Ltmp58:
	.loc	3 290 5
	lui	a2, 1
	cmove	ca0, cs0
	cmove	ca1, cs1
	ccall	_Z13vector_memcpyPhPKhm
.Ltmp59:
	.loc	3 0 5 is_stmt 0
	mv	a0, zero
	lui	a1, 2
	addiw	a1, a1, 16
	cincoffset	ca1, csp, a1
	csetbounds	ca1, ca1, 64
	addi	a2, zero, 128
.Ltmp60:
.LBB4_3:
	.loc	3 296 27 is_stmt 1
	slli	a4, a0, 5
	ori	a3, a4, 16
	cincoffset	ca3, cs0, a3
	clc	ca3, 0(ca3)
	.loc	3 296 37 is_stmt 0
	cld	a5, 0(ca3)
	.loc	3 296 60
	cincoffset	ca4, cs0, a4
	cld	a4, 0(ca4)
	bne	a5, a4, .LBB4_7
.Ltmp61:
.LBB4_10:
	.loc	3 299 78 is_stmt 1
	auipcc	ca4, %captab_pcrel_hi(.L__const._Z13run_base_testv.indices)
	clc	ca4, %pcrel_lo(.LBB4_10)(ca4)
	slli	a5, a0, 2
	cincoffset	ca4, ca4, a5
	clw	a4, 0(ca4)
	.loc	3 299 72 is_stmt 0
	slli	a4, a4, 4
	cincoffset	ca4, ca1, a4
.Ltmp62:
	.loc	3 0 72
	bne	a3, a4, .LBB4_7
.Ltmp63:
	addi	a0, a0, 1
.Ltmp64:
	.loc	3 293 5 is_stmt 1
	bne	a0, a2, .LBB4_3
.Ltmp65:
	.loc	3 0 5 is_stmt 0
	addi	a0, zero, 1
	j	.LBB4_8
.Ltmp66:
.LBB4_7:
	mv	a0, zero
.Ltmp67:
.LBB4_8:
	.loc	3 304 1 is_stmt 1
	lui	a1, 2
	addiw	a1, a1, -1904
	cincoffset	csp, csp, a1
	clc	cs1, 1984(csp)
	clc	cs0, 2000(csp)
	clc	cra, 2016(csp)
	cincoffset	csp, csp, 2032
	cret
.Ltmp68:
.Lfunc_end4:
	.size	_Z13run_base_testv, .Lfunc_end4-_Z13run_base_testv
	.cfi_endproc

	.globl	_Z19run_invalidate_testv
	.p2align	2
	.type	_Z19run_invalidate_testv,@function
_Z19run_invalidate_testv:
.Lfunc_begin5:
	.loc	3 307 0
	.cfi_startproc
	cincoffset	csp, csp, -2032
	.cfi_def_cfa_offset 2032
	csc	cra, 2016(csp)
	csc	cs0, 2000(csp)
	csc	cs1, 1984(csp)
	csc	cs2, 1968(csp)
	csc	cs3, 1952(csp)
	.cfi_offset ra, -16
	.cfi_offset s0, -32
	.cfi_offset s1, -48
	.cfi_offset s2, -64
	.cfi_offset s3, -80
	lui	a0, 1048574
	addiw	a0, a0, 1872
	cincoffset	csp, csp, a0
	.cfi_def_cfa_offset 8352
	mv	a0, zero
	lui	a1, 2
	addiw	a1, a1, 16
	cincoffset	ca1, csp, a1
	csetbounds	ca1, ca1, 64
.Ltmp69:
	.loc	3 311 10 prologue_end
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
.LBB5_1:
.LBB5_8:
.Ltmp70:
	.loc	3 341 21
	auipcc	ca3, %captab_pcrel_hi(.L__const._Z19run_invalidate_testv.indices)
	clc	ca3, %pcrel_lo(.LBB5_8)(ca3)
	slli	a4, a0, 2
	cincoffset	ca3, ca3, a4
	clw	a3, 0(ca3)
.Ltmp71:
	.loc	3 343 36
	slli	a3, a3, 4
	cincoffset	ca3, ca1, a3
	.loc	3 343 49 is_stmt 0
	cld	a4, 0(ca3)
	.loc	3 342 25 is_stmt 1
	slli	a5, a0, 5
	cincoffset	cs0, cs1, a5
	csd	a4, 0(cs0)
	ori	a4, a5, 16
	cincoffset	ca4, cs1, a4
.Ltmp72:
	.loc	3 340 34
	addi	a0, a0, 1
.Ltmp73:
	.loc	3 342 25
	csc	ca3, 0(ca4)
.Ltmp74:
	.loc	3 340 5
	bne	a0, a2, .LBB5_1
.Ltmp75:
	.loc	3 0 5 is_stmt 0
	lui	a0, 1
	cincoffset	ca1, csp, 16
	csetbounds	cs3, ca1, a0
	.loc	3 348 13 is_stmt 1
	lui	a2, 1
	cmove	ca0, cs3
	mv	a1, zero
	ccall	memset
.Ltmp76:
	.loc	3 354 5
	lui	a2, 1
	cmove	ca0, cs3
	cmove	ca1, cs1
	ccall	_Z24vector_memcpy_invalidatePhPKhm
.Ltmp77:
	.loc	3 359 59
	clc	ca0, 32(csp)
.Ltmp78:
	.loc	3 360 13
	ccall	_ZL13cheri_tag_getPv
.Ltmp79:
	mv	a1, a0
.Ltmp80:
	.loc	3 0 13 is_stmt 0
	mv	a0, zero
	bnez	a1, .LBB5_7
.Ltmp81:
	mv	s1, zero
	addi	s2, zero, 127
.Ltmp82:
.LBB5_4:
	mv	s0, s1
.Ltmp83:
	.loc	3 357 5 is_stmt 1
	beq	s1, s2, .LBB5_6
.Ltmp84:
	.loc	3 0 0 is_stmt 0
	addi	s1, s0, 1
.Ltmp85:
	.loc	3 359 59 is_stmt 1
	slli	a0, s1, 5
	ori	a0, a0, 16
	cincoffset	ca0, cs3, a0
	clc	ca0, 0(ca0)
.Ltmp86:
	.loc	3 360 13
	ccall	_ZL13cheri_tag_getPv
.Ltmp87:
	.loc	3 0 13 is_stmt 0
	beqz	a0, .LBB5_4
.Ltmp88:
.LBB5_6:
	addi	a0, zero, 126
	.loc	3 357 26 is_stmt 1
	sltu	a0, a0, s0
.Ltmp89:
.LBB5_7:
	.loc	3 365 1
	lui	a1, 2
	addiw	a1, a1, -1872
	cincoffset	csp, csp, a1
	clc	cs3, 1952(csp)
	clc	cs2, 1968(csp)
	clc	cs1, 1984(csp)
	clc	cs0, 2000(csp)
	clc	cra, 2016(csp)
	cincoffset	csp, csp, 2032
	cret
.Ltmp90:
.Lfunc_end5:
	.size	_Z19run_invalidate_testv, .Lfunc_end5-_Z19run_invalidate_testv
	.cfi_endproc

	.p2align	2
	.type	_ZL13cheri_tag_getPv,@function
_ZL13cheri_tag_getPv:
.Lfunc_begin6:
	.file	4 "/home/samuel/cheri/output/sdk/lib/clang/13.0.0/include" "cheri.h"
	.loc	4 82 0
	.cfi_startproc
	.loc	4 82 1 prologue_end
	cgettag	a0, ca0
.Ltmp91:
	cret
.Ltmp92:
.Lfunc_end6:
	.size	_ZL13cheri_tag_getPv, .Lfunc_end6-_ZL13cheri_tag_getPv
	.cfi_endproc

	.globl	main
	.p2align	2
	.type	main,@function
main:
.Lfunc_begin7:
	.loc	3 377 0
	.cfi_startproc
	cincoffset	csp, csp, -32
	.cfi_def_cfa_offset 32
.Ltmp93:
	.loc	3 0 0 prologue_end
	csc	cra, 16(csp)
	csc	cs0, 0(csp)
	.cfi_offset ra, -16
	.cfi_offset s0, -32
.Ltmp94:
	.loc	3 382 15
	ccall	_Z13run_base_testv
.Ltmp95:
	mv	s0, a0
.Ltmp96:
	.loc	3 386 15
	ccall	_Z19run_invalidate_testv
.Ltmp97:
.LBB7_1:
	.loc	3 389 25
	auipcc	ca1, %captab_pcrel_hi(outputAttempted)
	clc	ca1, %pcrel_lo(.LBB7_1)(ca1)
	addi	a2, zero, 3
	csw	a2, 0(ca1)
.LBB7_2:
	.loc	3 390 25
	auipcc	ca1, %captab_pcrel_hi(outputSucceeded)
	clc	ca1, %pcrel_lo(.LBB7_2)(ca1)
	.loc	3 386 37
	slli	a0, a0, 1
	.loc	3 386 12 is_stmt 0
	or	a2, a0, s0
.Ltmp98:
	.loc	3 391 5 is_stmt 1
	sext.w	a0, a2
	.loc	3 390 25
	csw	a2, 0(ca1)
	.loc	3 391 5
	clc	cs0, 0(csp)
	clc	cra, 16(csp)
	cincoffset	csp, csp, 32
	cret
.Ltmp99:
.Lfunc_end7:
	.size	main, .Lfunc_end7-main
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

	.type	.L__const._Z19run_invalidate_testv.indices,@object
	.p2align	2
.L__const._Z19run_invalidate_testv.indices:
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
	.size	.L__const._Z19run_invalidate_testv.indices, 512

	.file	5 "/home/samuel/cheri/output/sdk/lib/clang/13.0.0/include" "riscv_vector.h"
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
	.quad	.Ltmp35-.Lfunc_begin0
	.quad	.Ltmp36-.Lfunc_begin0
	.half	1
	.byte	94
	.quad	0
	.quad	0
.Ldebug_loc8:
	.quad	.Ltmp36-.Lfunc_begin0
	.quad	.Ltmp38-.Lfunc_begin0
	.half	1
	.byte	94
	.quad	0
	.quad	0
.Ldebug_loc9:
	.quad	.Ltmp37-.Lfunc_begin0
	.quad	.Ltmp42-.Lfunc_begin0
	.half	1
	.byte	95
	.quad	0
	.quad	0
.Ldebug_loc10:
	.quad	.Ltmp44-.Lfunc_begin0
	.quad	.Ltmp49-.Lfunc_begin0
	.half	1
	.byte	93
	.quad	0
	.quad	0
.Ldebug_loc11:
	.quad	.Ltmp45-.Lfunc_begin0
	.quad	.Ltmp49-.Lfunc_begin0
	.half	2
	.byte	144
	.byte	104
	.quad	0
	.quad	0
.Ldebug_loc12:
	.quad	.Ltmp55-.Lfunc_begin0
	.quad	.Ltmp57-.Lfunc_begin0
	.half	1
	.byte	90
	.quad	0
	.quad	0
.Ldebug_loc13:
	.quad	.Ltmp58-.Lfunc_begin0
	.quad	.Ltmp61-.Lfunc_begin0
	.half	3
	.byte	114
	.byte	16
	.byte	159
	.quad	.Ltmp63-.Lfunc_begin0
	.quad	.Lfunc_end4-.Lfunc_begin0
	.half	3
	.byte	114
	.byte	16
	.byte	159
	.quad	0
	.quad	0
.Ldebug_loc14:
	.quad	.Ltmp58-.Lfunc_begin0
	.quad	.Ltmp61-.Lfunc_begin0
	.half	4
	.byte	114
	.byte	144
	.byte	32
	.byte	159
	.quad	.Ltmp63-.Lfunc_begin0
	.quad	.Lfunc_end4-.Lfunc_begin0
	.half	4
	.byte	114
	.byte	144
	.byte	32
	.byte	159
	.quad	0
	.quad	0
.Ldebug_loc15:
	.quad	.Ltmp60-.Lfunc_begin0
	.quad	.Ltmp61-.Lfunc_begin0
	.half	1
	.byte	90
	.quad	.Ltmp64-.Lfunc_begin0
	.quad	.Ltmp65-.Lfunc_begin0
	.half	1
	.byte	90
	.quad	0
	.quad	0
.Ldebug_loc16:
	.quad	.Ltmp73-.Lfunc_begin0
	.quad	.Ltmp75-.Lfunc_begin0
	.half	1
	.byte	90
	.quad	0
	.quad	0
.Ldebug_loc17:
	.quad	.Ltmp76-.Lfunc_begin0
	.quad	.Lfunc_end5-.Lfunc_begin0
	.half	3
	.byte	114
	.byte	16
	.byte	159
	.quad	0
	.quad	0
.Ldebug_loc18:
	.quad	.Ltmp76-.Lfunc_begin0
	.quad	.Lfunc_end5-.Lfunc_begin0
	.half	4
	.byte	114
	.byte	144
	.byte	32
	.byte	159
	.quad	0
	.quad	0
.Ldebug_loc19:
	.quad	.Ltmp77-.Lfunc_begin0
	.quad	.Ltmp80-.Lfunc_begin0
	.half	2
	.byte	48
	.byte	159
	.quad	.Ltmp80-.Lfunc_begin0
	.quad	.Ltmp82-.Lfunc_begin0
	.half	2
	.byte	49
	.byte	159
	.quad	.Ltmp83-.Lfunc_begin0
	.quad	.Ltmp87-.Lfunc_begin0
	.half	6
	.byte	120
	.byte	0
	.byte	17
	.byte	1
	.byte	34
	.byte	159
	.quad	.Ltmp87-.Lfunc_begin0
	.quad	.Ltmp88-.Lfunc_begin0
	.half	6
	.byte	120
	.byte	0
	.byte	17
	.byte	2
	.byte	34
	.byte	159
	.quad	0
	.quad	0
.Ldebug_loc20:
	.quad	.Lfunc_begin6-.Lfunc_begin0
	.quad	.Ltmp91-.Lfunc_begin0
	.half	1
	.byte	90
	.quad	0
	.quad	0
.Ldebug_loc21:
	.quad	.Ltmp94-.Lfunc_begin0
	.quad	.Ltmp96-.Lfunc_begin0
	.half	3
	.byte	17
	.byte	0
	.byte	159
	.quad	.Ltmp96-.Lfunc_begin0
	.quad	.Ltmp97-.Lfunc_begin0
	.half	1
	.byte	88
	.quad	.Ltmp98-.Lfunc_begin0
	.quad	.Lfunc_end7-.Lfunc_begin0
	.half	1
	.byte	92
	.quad	0
	.quad	0
.Ldebug_loc22:
	.quad	.Ltmp94-.Lfunc_begin0
	.quad	.Ltmp96-.Lfunc_begin0
	.half	3
	.byte	17
	.byte	1
	.byte	159
	.quad	.Ltmp96-.Lfunc_begin0
	.quad	.Ltmp97-.Lfunc_begin0
	.half	3
	.byte	17
	.byte	3
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
	.byte	2
	.byte	24
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
	.byte	17
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
	.byte	18
	.byte	52
	.byte	0
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
	.byte	19
	.byte	11
	.byte	1
	.byte	85
	.byte	23
	.byte	0
	.byte	0
	.byte	20
	.ascii	"\211\202\001"
	.byte	0
	.byte	49
	.byte	19
	.byte	17
	.byte	1
	.byte	0
	.byte	0
	.byte	21
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
	.byte	5
	.byte	73
	.byte	19
	.byte	63
	.byte	25
	.byte	0
	.byte	0
	.byte	22
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
	.byte	0
	.byte	0
	.byte	23
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
	.byte	24
	.byte	15
	.byte	0
	.byte	11
	.byte	11
	.byte	0
	.byte	0
	.byte	25
	.byte	38
	.byte	0
	.byte	0
	.byte	0
	.byte	26
	.byte	22
	.byte	0
	.byte	73
	.byte	19
	.byte	3
	.byte	14
	.byte	0
	.byte	0
	.byte	27
	.byte	1
	.byte	1
	.ascii	"\207B"
	.byte	25
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	28
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
	.byte	29
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
	.byte	30
	.byte	1
	.byte	1
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	31
	.byte	33
	.byte	0
	.byte	73
	.byte	19
	.byte	55
	.byte	11
	.byte	0
	.byte	0
	.byte	32
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
	.word	.Lfunc_end7-.Lfunc_begin0
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
	.byte	6
	.byte	5
	.word	.Linfo_string8
	.byte	16
	.byte	3
	.byte	131
	.byte	16
	.byte	7
	.word	.Linfo_string6
	.word	113
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
	.word	130
	.byte	16
	.byte	5
	.word	42
	.word	.Linfo_string9
	.byte	2
	.byte	226
	.byte	3
	.word	147
	.byte	16
	.byte	4
	.word	130
	.byte	8
	.quad	.Lfunc_begin0
	.word	.Lfunc_end0-.Lfunc_begin0
	.byte	1
	.byte	82

	.word	.Linfo_string10
	.byte	3
	.byte	8
	.word	1290

	.byte	9
	.byte	1
	.byte	90
	.word	.Linfo_string25
	.byte	3
	.byte	8
	.word	1290
	.byte	9
	.byte	1
	.byte	91
	.word	.Linfo_string26
	.byte	3
	.byte	8
	.word	1292
	.byte	9
	.byte	1
	.byte	92
	.word	.Linfo_string27
	.byte	3
	.byte	8
	.word	66
	.byte	10
	.byte	3
	.byte	123
	.byte	0
	.byte	159
	.word	.Linfo_string28
	.byte	3
	.byte	9
	.word	42
	.byte	10
	.byte	1
	.byte	90
	.word	.Linfo_string29
	.byte	3
	.byte	10
	.word	49
	.byte	11
	.quad	.Lfunc_begin0
	.word	.Ltmp5-.Lfunc_begin0
	.byte	12
	.word	.Ldebug_loc0
	.word	.Linfo_string30
	.byte	3
	.byte	11
	.word	1292
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
	.word	1290

	.byte	9
	.byte	1
	.byte	90
	.word	.Linfo_string25
	.byte	3
	.byte	19
	.word	1290
	.byte	13
	.word	.Ldebug_loc1
	.word	.Linfo_string31
	.byte	3
	.byte	19
	.word	1306
	.byte	9
	.byte	1
	.byte	92
	.word	.Linfo_string27
	.byte	3
	.byte	19
	.word	66
	.byte	12
	.word	.Ldebug_loc2
	.word	.Linfo_string29
	.byte	3
	.byte	20
	.word	49
	.byte	10
	.byte	1
	.byte	91
	.word	.Linfo_string32
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
	.byte	153

	.byte	9
	.byte	1
	.byte	90
	.word	.Linfo_string33
	.byte	3
	.byte	153
	.word	124
	.byte	9
	.byte	1
	.byte	91
	.word	.Linfo_string31
	.byte	3
	.byte	153
	.word	141
	.byte	9
	.byte	1
	.byte	92
	.word	.Linfo_string34
	.byte	3
	.byte	153
	.word	66
	.byte	11
	.quad	.Ltmp16
	.word	.Ltmp22-.Ltmp16
	.byte	12
	.word	.Ldebug_loc3
	.word	.Linfo_string35
	.byte	3
	.byte	157
	.word	66
	.byte	12
	.word	.Ldebug_loc4
	.word	.Linfo_string36
	.byte	3
	.byte	158
	.word	66
	.byte	0
	.byte	11
	.quad	.Ltmp24
	.word	.Ltmp29-.Ltmp24
	.byte	12
	.word	.Ldebug_loc5
	.word	.Linfo_string37
	.byte	3
	.byte	173
	.word	66
	.byte	12
	.word	.Ldebug_loc6
	.word	.Linfo_string38
	.byte	3
	.byte	175
	.word	1313
	.byte	0
	.byte	0
	.byte	14
	.quad	.Lfunc_begin3
	.word	.Lfunc_end3-.Lfunc_begin3
	.byte	1
	.byte	82

	.word	.Linfo_string14
	.word	.Linfo_string15
	.byte	3
	.byte	193

	.byte	9
	.byte	1
	.byte	90
	.word	.Linfo_string33
	.byte	3
	.byte	193
	.word	124
	.byte	9
	.byte	1
	.byte	91
	.word	.Linfo_string31
	.byte	3
	.byte	193
	.word	141
	.byte	9
	.byte	1
	.byte	92
	.word	.Linfo_string34
	.byte	3
	.byte	193
	.word	66
	.byte	11
	.quad	.Ltmp34
	.word	.Ltmp41-.Ltmp34
	.byte	12
	.word	.Ldebug_loc7
	.word	.Linfo_string35
	.byte	3
	.byte	196
	.word	66
	.byte	12
	.word	.Ldebug_loc8
	.word	.Linfo_string36
	.byte	3
	.byte	197
	.word	66
	.byte	12
	.word	.Ldebug_loc9
	.word	.Linfo_string42
	.byte	3
	.byte	208
	.word	66
	.byte	0
	.byte	11
	.quad	.Ltmp43
	.word	.Ltmp48-.Ltmp43
	.byte	12
	.word	.Ldebug_loc10
	.word	.Linfo_string37
	.byte	3
	.byte	225
	.word	66
	.byte	12
	.word	.Ldebug_loc11
	.word	.Linfo_string38
	.byte	3
	.byte	227
	.word	1313
	.byte	0
	.byte	0
	.byte	15
	.quad	.Lfunc_begin4
	.word	.Lfunc_end4-.Lfunc_begin4
	.byte	1
	.byte	82

	.word	.Linfo_string16
	.word	.Linfo_string17
	.byte	3
	.byte	243
	.word	1292

	.byte	10
	.byte	4
	.byte	145
	.asciz	"\220\300"
	.word	.Linfo_string43
	.byte	3
	.byte	247
	.word	1361
	.byte	16
	.byte	3
	.byte	145
	.ascii	"\220 "
	.word	.Linfo_string44
	.byte	3
	.half	275
	.word	1373
	.byte	16
	.byte	2
	.byte	145
	.byte	16
	.word	.Linfo_string48
	.byte	3
	.half	284
	.word	1373
	.byte	17
	.word	.Ldebug_loc13
	.word	.Linfo_string49
	.byte	3
	.half	287
	.word	1421
	.byte	17
	.word	.Ldebug_loc14
	.word	.Linfo_string50
	.byte	3
	.half	286
	.word	1421
	.byte	18
	.word	.Linfo_string51
	.byte	3
	.half	256
	.word	1427
	.byte	11
	.quad	.Ltmp52
	.word	.Ltmp57-.Ltmp52
	.byte	17
	.word	.Ldebug_loc12
	.word	.Linfo_string30
	.byte	3
	.half	276
	.word	66
	.byte	19
	.word	.Ldebug_ranges0
	.byte	18
	.word	.Linfo_string52
	.byte	3
	.half	277
	.word	1292
	.byte	0
	.byte	0
	.byte	11
	.quad	.Ltmp60
	.word	.Ltmp65-.Ltmp60
	.byte	17
	.word	.Ldebug_loc15
	.word	.Linfo_string30
	.byte	3
	.half	293
	.word	66
	.byte	0
	.byte	20
	.word	369
	.quad	.Ltmp59
	.byte	0
	.byte	21
	.quad	.Lfunc_begin5
	.word	.Lfunc_end5-.Lfunc_begin5
	.byte	1
	.byte	82

	.word	.Linfo_string19
	.word	.Linfo_string20
	.byte	3
	.half	307
	.word	1292

	.byte	16
	.byte	4
	.byte	145
	.asciz	"\220\300"
	.word	.Linfo_string43
	.byte	3
	.half	311
	.word	1361
	.byte	16
	.byte	3
	.byte	145
	.ascii	"\220 "
	.word	.Linfo_string44
	.byte	3
	.half	339
	.word	1373
	.byte	16
	.byte	2
	.byte	145
	.byte	16
	.word	.Linfo_string48
	.byte	3
	.half	348
	.word	1373
	.byte	17
	.word	.Ldebug_loc17
	.word	.Linfo_string49
	.byte	3
	.half	351
	.word	1421
	.byte	17
	.word	.Ldebug_loc18
	.word	.Linfo_string50
	.byte	3
	.half	350
	.word	1421
	.byte	18
	.word	.Linfo_string51
	.byte	3
	.half	320
	.word	1427
	.byte	11
	.quad	.Ltmp70
	.word	.Ltmp75-.Ltmp70
	.byte	17
	.word	.Ldebug_loc16
	.word	.Linfo_string30
	.byte	3
	.half	340
	.word	66
	.byte	19
	.word	.Ldebug_ranges1
	.byte	18
	.word	.Linfo_string52
	.byte	3
	.half	341
	.word	1292
	.byte	0
	.byte	0
	.byte	11
	.quad	.Ltmp77
	.word	.Ltmp89-.Ltmp77
	.byte	17
	.word	.Ldebug_loc19
	.word	.Linfo_string30
	.byte	3
	.half	357
	.word	66
	.byte	19
	.word	.Ldebug_ranges2
	.byte	18
	.word	.Linfo_string53
	.byte	3
	.half	359
	.word	84
	.byte	0
	.byte	0
	.byte	20
	.word	522
	.quad	.Ltmp77
	.byte	20
	.word	1160
	.quad	.Ltmp79
	.byte	20
	.word	1160
	.quad	.Ltmp87
	.byte	0
	.byte	22
	.quad	.Lfunc_begin6
	.word	.Lfunc_end6-.Lfunc_begin6
	.byte	1
	.byte	82

	.word	.Linfo_string21
	.word	.Linfo_string22
	.byte	4
	.byte	82
	.word	1299
	.byte	13
	.word	.Ldebug_loc20
	.word	.Linfo_string54
	.byte	4
	.byte	82
	.word	1290
	.byte	0
	.byte	23
	.quad	.Lfunc_begin7
	.word	.Lfunc_end7-.Lfunc_begin7
	.byte	1
	.byte	82

	.word	.Linfo_string24
	.byte	3
	.half	376
	.word	1292

	.byte	17
	.word	.Ldebug_loc21
	.word	.Linfo_string55
	.byte	3
	.half	378
	.word	1292
	.byte	17
	.word	.Ldebug_loc22
	.word	.Linfo_string56
	.byte	3
	.half	379
	.word	1292
	.byte	20
	.word	690
	.quad	.Ltmp95
	.byte	20
	.word	902
	.quad	.Ltmp97
	.byte	0
	.byte	24
	.byte	16
	.byte	2
	.word	.Linfo_string18
	.byte	5
	.byte	4
	.byte	2
	.word	.Linfo_string23
	.byte	2
	.byte	1
	.byte	3
	.word	1312
	.byte	16
	.byte	25
	.byte	5
	.word	1324
	.word	.Linfo_string41
	.byte	5
	.byte	99
	.byte	26
	.word	1333
	.word	.Linfo_string40
	.byte	27

	.word	42
	.byte	28
	.word	1354
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
	.byte	29
	.word	.Linfo_string39
	.byte	8
	.byte	7
	.byte	30
	.word	90
	.byte	31
	.word	1354
	.byte	4
	.byte	0
	.byte	30
	.word	1385
	.byte	31
	.word	1354
	.byte	128
	.byte	0
	.byte	6
	.byte	5
	.word	.Linfo_string47
	.byte	32
	.byte	3
	.byte	136
	.byte	16
	.byte	7
	.word	.Linfo_string45
	.word	113
	.byte	3
	.byte	138
	.byte	0
	.byte	32
	.word	.Linfo_string46
	.word	84
	.byte	3
	.byte	140
	.byte	16
	.byte	16
	.byte	0
	.byte	3
	.word	1385
	.byte	16
	.byte	30
	.word	1292
	.byte	31
	.word	1354
	.byte	128
	.byte	0
	.byte	0
.Ldebug_info_end0:
	.section	.debug_ranges,"",@progbits
.Ldebug_ranges0:
	.quad	.Ltmp52-.Lfunc_begin0
	.quad	.Ltmp54-.Lfunc_begin0
	.quad	.Ltmp55-.Lfunc_begin0
	.quad	.Ltmp56-.Lfunc_begin0
	.quad	0
	.quad	0
.Ldebug_ranges1:
	.quad	.Ltmp70-.Lfunc_begin0
	.quad	.Ltmp72-.Lfunc_begin0
	.quad	.Ltmp73-.Lfunc_begin0
	.quad	.Ltmp74-.Lfunc_begin0
	.quad	0
	.quad	0
.Ldebug_ranges2:
	.quad	.Ltmp77-.Lfunc_begin0
	.quad	.Ltmp81-.Lfunc_begin0
	.quad	.Ltmp85-.Lfunc_begin0
	.quad	.Ltmp88-.Lfunc_begin0
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
	.asciz	"_Z24vector_memcpy_invalidatePhPKhm"
.Linfo_string15:
	.asciz	"vector_memcpy_invalidate"
.Linfo_string16:
	.asciz	"_Z13run_base_testv"
.Linfo_string17:
	.asciz	"run_base_test"
.Linfo_string18:
	.asciz	"int"
.Linfo_string19:
	.asciz	"_Z19run_invalidate_testv"
.Linfo_string20:
	.asciz	"run_invalidate_test"
.Linfo_string21:
	.asciz	"_ZL13cheri_tag_getPv"
.Linfo_string22:
	.asciz	"cheri_tag_get"
.Linfo_string23:
	.asciz	"bool"
.Linfo_string24:
	.asciz	"main"
.Linfo_string25:
	.asciz	"dest"
.Linfo_string26:
	.asciz	"ch"
.Linfo_string27:
	.asciz	"count"
.Linfo_string28:
	.asciz	"ch_uc"
.Linfo_string29:
	.asciz	"dest_uc"
.Linfo_string30:
	.asciz	"i"
.Linfo_string31:
	.asciz	"src"
.Linfo_string32:
	.asciz	"src_uc"
.Linfo_string33:
	.asciz	"dst"
.Linfo_string34:
	.asciz	"num_bytes"
.Linfo_string35:
	.asciz	"num_elements"
.Linfo_string36:
	.asciz	"copied_128bit_elems_per_iter"
.Linfo_string37:
	.asciz	"copied_per_iter"
.Linfo_string38:
	.asciz	"data"
.Linfo_string39:
	.asciz	"__ARRAY_SIZE_TYPE__"
.Linfo_string40:
	.asciz	"__rvv_uint8m8_t"
.Linfo_string41:
	.asciz	"vuint8m8_t"
.Linfo_string42:
	.asciz	"num_64bit_elements"
.Linfo_string43:
	.asciz	"bases"
.Linfo_string44:
	.asciz	"source_array"
.Linfo_string45:
	.asciz	"expected_base_value"
.Linfo_string46:
	.asciz	"base_ptr"
.Linfo_string47:
	.asciz	"Element"
.Linfo_string48:
	.asciz	"dest_array"
.Linfo_string49:
	.asciz	"dst_ptr"
.Linfo_string50:
	.asciz	"src_ptr"
.Linfo_string51:
	.asciz	"indices"
.Linfo_string52:
	.asciz	"index"
.Linfo_string53:
	.asciz	"ptr"
.Linfo_string54:
	.asciz	"__cap"
.Linfo_string55:
	.asciz	"result"
.Linfo_string56:
	.asciz	"attempted"
	.ident	"clang version 13.0.0 (ssh://git@github.com/theturboturnip/llvm-project.git 88213dcf1e9bc454f471b9e9a8b2ede325dc5e24)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym outputAttempted
	.addrsig_sym outputSucceeded
	.section	.debug_line,"",@progbits
.Lline_table_start0:
