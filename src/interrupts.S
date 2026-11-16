/* src/interrupts.S */
.intel_syntax noprefix

.global isr0
.global isr1
.global isr2
.global isr3
.global isr4
.global isr5
.global isr6
.global isr7
.global isr8
.global isr9
.global isr10
.global isr11
.global isr12
.global isr13
.global isr14
.global isr15
.global isr16
.global isr17
.global isr18
.global isr19
.global isr20
.global isr21
.global isr22
.global isr23
.global isr24
.global isr25
.global isr26
.global isr27
.global isr28
.global isr29
.global isr30
.global isr31
.global irq0
.global irq1

.macro ISR_NO_ERRCODE num
    .globl isr\num
isr\num:
    pushad                # push eax, ecx, edx, ebx, esp, ebp, esi, edi
    call isr\num        # call the Rust handler (same name)
    popad                 # restore registers
    add esp, 0            # no error code to discard
    iret
.endm

.macro ISR_ERRCODE num
    .globl isr\num
isr\num:
    push dword [esp]      # error code is already on stack, keep it
    pushad
    call isr\num        # Rust handler
    popad
    add esp, 4            # discard error code
    iret
.endm

/* Exceptions without error code */
ISR_NO_ERRCODE 0
ISR_NO_ERRCODE 1
ISR_NO_ERRCODE 2
ISR_NO_ERRCODE 3
ISR_NO_ERRCODE 4
ISR_NO_ERRCODE 5
ISR_NO_ERRCODE 6
ISR_NO_ERRCODE 7
ISR_NO_ERRCODE 9
ISR_NO_ERRCODE 10
ISR_NO_ERRCODE 11
ISR_NO_ERRCODE 12
ISR_NO_ERRCODE 13
ISR_NO_ERRCODE 14
ISR_NO_ERRCODE 16
ISR_NO_ERRCODE 17
ISR_NO_ERRCODE 18
ISR_NO_ERRCODE 19
ISR_NO_ERRCODE 20
ISR_NO_ERRCODE 21
ISR_NO_ERRCODE 22
ISR_NO_ERRCODE 23
ISR_NO_ERRCODE 24
ISR_NO_ERRCODE 25
ISR_NO_ERRCODE 26
ISR_NO_ERRCODE 27
ISR_NO_ERRCODE 28
ISR_NO_ERRCODE 29
ISR_NO_ERRCODE 30
ISR_NO_ERRCODE 31

/* Exceptions with error code – for completeness we treat same as no‑err */
ISR_NO_ERRCODE 8
ISR_NO_ERRCODE 15

/* IRQ handlers */
irq0:
    pushad
    call irq0          # Rust handler defined as `irq0` in idt.rs
    popad
    iret

irq1:
    pushad
    call irq1          # Rust handler defined as `irq1` in idt.rs
    popad
    iret
