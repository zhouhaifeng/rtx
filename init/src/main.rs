


fn main()
{
    mem_init(main_memory_start,memory_end); // 主内存区初始化。mm/memory.c
	trap_init();                            // 陷阱门(硬件中断向量)初始化，kernel/traps.c
	blk_dev_init();                         // 块设备初始化,kernel/blk_drv/ll_rw_blk.c
	chr_dev_init();                         // 字符设备初始化, kernel/chr_drv/tty_io.c
	tty_init();                             // tty初始化， kernel/chr_drv/tty_io.c
	time_init();                            // 设置开机启动时间 startup_time
	sched_init();                           // 调度程序初始化(加载任务0的tr,ldtr)(kernel/sched.c)
    // 缓冲管理初始化，建内存链表等。(fs/buffer.c)
	buffer_init(buffer_memory_end);
	hd_init();                              // 硬盘初始化，kernel/blk_drv/hd.c
	floppy_init();                          // 软驱初始化，kernel/blk_drv/floppy.c
	sti();                                  // 所有初始化工作都做完了，开启中断
    // 下面过程通过在堆栈中设置的参数，利用中断返回指令启动任务0执行。
	move_to_user_mode();                    // 移到用户模式下执行
	if (!fork()) {		/* we count on this going ok */
		init();                             // 在新建的子进程(任务1)中执行。
	}
}

fn init()
{
    
}