use parser::MonkeyAST;

pub fn wrap_gcc_assembly_file(ast: MonkeyAST) -> String {
    let gcc_assembly_mo = format!(
        "
.file	\"mk.c\"
.text
.globl	main
.type	main, @function
_monkey_lang_start:
{}

main:
.LFB0:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movl	%edi, -4(%rbp)
	movq	%rsi, -16(%rbp)
	call _monkey_lang_start
	leave

	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE0:
	.size	main, .-main
	.section	.rodata
.LC0:
	.string	{}
	.text
	.globl	_monkey_do_output
	.type	_monkey_do_output, @function
_monkey_do_output:
.LFB1:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movl	%edi, -4(%rbp)
	movl	-4(%rbp), %eax
	movl	%eax, %esi
	leaq	.LC0(%rip), %rdi
	movl	$0, %eax
	call	printf@PLT
	nop
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE1:
	.size	_monkey_do_output, .-_monkey_do_output
	.section	.rodata
.LC1:
	.string	{}
	.text
	.globl	_monkey_do_ascii_output
	.type	_monkey_do_ascii_output, @function
_monkey_do_ascii_output:
.LFB2:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movl	%edi, -4(%rbp)
	movl	-4(%rbp), %eax
	movl	%eax, %esi
	leaq	.LC1(%rip), %rdi
	movl	$0, %eax
	call	printf@PLT
	nop
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE2:
	.size	_monkey_do_ascii_output, .-_monkey_do_ascii_output
	.ident	\"GCC: (GNU) 7.1.1 20170630\"
	.section	.note.GNU-stack,\"\",@progbits
",
        "{}",
        stringify!("%i\n"),
        stringify!("char:%i\n")
    );
    println!("{}", gcc_assembly_mo);
    let mut ret = String::new();
    ret
}

fn build_command_O() -> String {
    let gcc_assembly_mo = "
movl	{}, %edi
call	_monkey_do_output
    ";
	String::new()
}

fn build_command_AO() -> String {
    let gcc_assembly_mo = "
movl	{}, %edi
call	_monkey_do_ascii_output    
    ";
	String::new()
}