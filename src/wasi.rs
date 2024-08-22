use voxel_engine_types::*;

// Creates the function item `add_wasi_snapshot_preview1_to_wasmi_linker` which when called adds all
// `wasi preview_1` functions to the linker
macro_rules! impl_trap_for_funcs {
    (
        $(
            $( #[$docs:meta] )*
            fn $fname:ident ($( $arg:ident : $typ:ty ),* $(,)? ) -> $ret:tt
        );+ $(;)?
    ) => {
        $(
            $( #[$docs] )*
            #[allow(warnings)]
            #[no_mangle]
            pub extern "C" fn $fname ($( $arg : $typ ),* ) -> $ret {
                unimplemented!()
            }
        )+
    }
}

/// Represents an independent vector of output data.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Ciovec {
    /// The address of the buffer to be written.
    pub buf: *const u8,
    /// The length of the buffer to be written.
    pub buf_len: usize,
}

/// Read environment variable data.
///
/// # Note
///
/// The sizes of the buffers should match that returned by `environ_sizes_get`.
/// Key/value pairs are expected to be joined with =s, and terminated with \0s.
/// 
/// # Safety
/// 
/// The input pointers must be valid.
#[export_name = "__imported_wasi_snapshot_preview1_environ_get"]
pub unsafe extern "C" fn environ_get(_: *mut *mut u8, _: *mut u8) -> i32 {
    0
}

/// Returns the number of environment variables.
///
/// # Note
///
/// Returns the number of environment variable arguments and the size of the environment variable data.
/// Note that `offset0` and `offset1` are offsets into memory where the two results are stored.
/// 
/// # Safety
/// 
/// The input pointers must be valid.
#[export_name = "__imported_wasi_snapshot_preview1_environ_sizes_get"]
pub unsafe extern "C" fn environ_sizes_get(offset0: *mut usize, offset1: *mut usize) -> i32 {
    *offset0 = 0;
    *offset1 = 0;
    0
}

/// Write to a file descriptor.
///
/// # Note
///
/// This is similar to `writev` in POSIX.
///
/// # Parameters
///
/// - `fd`: The file descriptor.
/// - `ciov_buf`, `ciov_buf_len`: used to create ciovec, which is the list of
///                               scatter/gather vectors from which to retrieve data.
/// - `offset0`: The offset into the memory where result (size written) is stored
/// 
/// # Safety
/// 
/// The inputs must point to valid buffers and the buffer lengths must be valid.
#[export_name = "_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17h594e175a549b8f2dE"]
pub unsafe extern "C" fn fd_write(fd: i32, mut ciov_buf: *const Ciovec, mut ciov_buf_len: usize, nwritten: *mut usize) -> i32 {
    static mut BUFFER: String = String::new();

    if fd == 1 || fd == 2 {

        while ciov_buf_len != 0 && (*ciov_buf).buf_len == 0 {
            ciov_buf = ciov_buf.add(1);
            ciov_buf_len -= 1;
        }

        *nwritten = 0;
        let current_string = &mut *core::ptr::addr_of_mut!(BUFFER);
        let log_level = if fd == 1 { LogLevel::Info } else { LogLevel::Error };

        while ciov_buf_len > 0 {
            let mut to_write = core::str::from_utf8_unchecked(core::slice::from_raw_parts((*ciov_buf).buf, (*ciov_buf).buf_len));
            while let Some(last) = to_write.find('\n') {
                let sequence = &to_write[..last];
                to_write = &to_write[(last + 1)..];
                if current_string.is_empty() {
                    global_log(log_level, sequence);
                }
                else {
                    *current_string += sequence;
                    global_log(log_level, current_string);
                    current_string.clear();
                }
                *nwritten += sequence.len() + 1;
            }
            *nwritten += to_write.len();
            *current_string += to_write;
            ciov_buf_len -= 1;
        }

        0
    } 
    else {
        unreachable!()
    }
}

/// Terminate the process normally.
///
/// # Note
///
/// An exit code of 0 indicates successful termination of the program.
/// The meanings of other values is dependent on the environment.
///
/// # Parameters
///
/// - `rval`: The exit code returned by the process.
#[export_name = "__imported_wasi_snapshot_preview1_proc_exit"]
fn proc_exit(_: i32) {
    unreachable!()
}

/// Temporarily yield execution of the calling thread.
///
/// # Note
///
/// This is similar to sched_yield in POSIX.
#[export_name = "_ZN4wasi13lib_generated22wasi_snapshot_preview111sched_yield17hd8d25f53c5eb182fE"]
fn sched_yield() -> i32 {
    0
}

/// Write high-quality random data into a buffer.
///
/// # Parameters
///
/// - `buf`: The buffer to fill with random data.
/// - `buf_len`: The length of the `buf` buffer.
#[export_name = "_ZN4wasi13lib_generated22wasi_snapshot_preview110random_get17hb1719c7a33a320e4E"]
unsafe fn random_get(buf: *mut u8, buf_len: usize) -> i32 {
    // todo: ask host for randomness
    std::slice::from_raw_parts_mut(buf, buf_len).fill(0);
    0
}

impl_trap_for_funcs!(
    /// Read command-line argument data.
    ///
    /// # Note
    ///
    /// The size of the array should match that returned by `args_sizes_get`.
    /// Each argument is expected to be \0 terminated.
    fn args_get(argv: i32, argv_buf: i32) -> i32;

    /// Return command-line argument data sizes.
    ///
    /// # Note
    ///
    /// Returns the number of arguments and the size of the argument string data, or an error.
    /// Note that `offset0` and `offset1` are offsets into memory where the two results are stored
    fn args_sizes_get(offset0: i32, offset1: i32) -> i32;

    /*    /// Read environment variable data.
    ///
    /// # Note
    ///
    /// The sizes of the buffers should match that returned by `environ_sizes_get`.
    /// Key/value pairs are expected to be joined with =s, and terminated with \0s.
    fn environ_get(environ: i32, environ_buf: i32) -> i32;

    /// Returns the number of environment variables.
    ///
    /// # Note
    ///
    /// Returns the number of environment variable arguments and the size of the environment variable data.
    /// Note that `offset0` and `offset1` are offsets into memory where the two results are stored.
    fn environ_sizes_get(offset0: i32, offset1: i32) -> i32; */


    /// Return the resolution of a clock.
    ///
    /// Implementations are required to provide a non-zero value for supported clocks.
    /// For unsupported clocks, return `errno::inval`.
    ///
    /// # Note
    ///
    /// This is similar to `clock_getres` in POSIX.
    /// The `id` is the `ClockID` and `offset0` is the offset into memory where the result is written.
    fn clock_res_get(id: i32, offset0: i32) -> i32;

    /// Return the time value of a clock.
    ///
    /// # Note
    ///
    /// This is similar to `clock_gettime` in POSIX. The result is stored in `offset0`.
    #[export_name = "_ZN4wasi13lib_generated22wasi_snapshot_preview114clock_time_get17h35e4b5c443113208E"]
    fn clock_time_get(id: i32, precision: i64, offset0: i32) -> i32;

    /// Provide file advisory information on a file descriptor.
    ///
    /// # Note
    ///
    /// This is similar to `posix_fadvise` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `offset`: The offset within the file to which the advisory applies.
    /// - `len`: The length of the region to which the advisory applies.
    /// - `advice`: The advice.
    fn fd_advise(fd: i32, offset: i64, len: i64, advice: i32) -> i32;

    /// Force the allocation of space in a file.
    ///
    /// # Note
    ///
    /// This is similar to `posix_fallocate` in `POSIX`.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `offset`: The offset at which to start the allocation.
    /// - `len`: The length of the area that is allocated.
    fn fd_allocate(fd: i32, offset: i64, len: i64) -> i32;

    /// Close a file descriptor.
    ///
    /// # Note
    ///
    /// This is similar to `close` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor that shall be closed.
    fn fd_close(fd: i32) -> i32;

    /// Synchronize the data of a file to disk.
    ///
    /// # Note
    ///
    /// This is similar to `fdatasync` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor of the file to be synchronized to disk.
    fn fd_datasync(fd: i32) -> i32;

    /// Get the attributes of a file descriptor.
    ///
    /// # Note
    ///
    /// This returns similar flags to `fsync(fd, F_GETFL)` in POSIX, as well as additional fields.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `offset0`: The offset into memory where the result is written to.
    fn fd_fdstat_get(fd: i32, offset0: i32) -> i32;

    /// Adjust the flags associated with a file descriptor.
    ///
    /// # Note
    ///
    /// This is similar to `fcntl(fd, F_SETFL, flags)` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `flags`: The desired values of the file descriptor flags.
    fn fd_fdstat_set_flags(fd: i32, flags: i32) -> i32;

    /// Adjust the rights associated with a file descriptor.
    ///
    /// # Note
    ///
    /// This can only be used to remove rights, and returns `errno::notcapable`
    /// if called in a way that would attempt to add rights.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `fs_rights_base`: The desired rights of the file descriptor.
    /// - `fs_rights_inheriting`: The inherited rights.
    fn fd_fdstat_set_rights(fd: i32, fs_rights_base: i64, fs_rights_inheriting: i64) -> i32;

    /// Returns the attributes of an open file.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `offset0`: The offset into memory where the buffer of the file's attributes is written.
    fn fd_filestat_get(fd: i32, offset0: i32) -> i32;

    /// Adjust the size of an open file.
    ///
    /// # Note
    ///
    /// - If this increases the file's size, the extra bytes are filled with zeros.
    /// - This is similar to `ftruncate` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `size`: The desired file size.
    fn fd_filestat_set_size(fd: i32, size: i64) -> i32;

    /// Adjust the timestamps of an open file or directory.
    ///
    /// # Note
    ///
    /// This is similar to `futimens` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `atim`: The desired values of the data access timestamp.
    /// - `mtim`: The desired values of the data modification timestamp.
    /// - `fst_flags`: A bitmask indicating which timestamps to adjust.
    fn fd_filestat_set_times(fd: i32, atim: i64, mtim: i64, fst_flags: i32) -> i32;

    /// Read from a file descriptor, without using and updating the file descriptor's offset.
    ///
    /// # Note
    ///
    /// This is similar to `preadv` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `iov_buf`, `iov_buf_len`: Used to create `iovec`,
    ///                             which is the list of scatter/gather vectors in which to store data.
    /// - `offset`: The offset within the file at which to read.
    /// - `offsset0`: The size of bytes read is written here.
    fn fd_pread(fd: i32, iov_buf: i32, iov_buf_len: i32, offset: i64, offset0: i32) -> i32;

    /// Return a description of the given preopened file descriptor.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `offset0`: The location in the memory where the buffer that stores the description is written.
    fn fd_prestat_get(fd: i32, offset0: i32) -> i32;

    /// Return a description of the given preopened file descriptor.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `path`: A buffer into which to write the preopened directory name.
    /// - `path_len`: The length of the `path` buffer.
    fn fd_prestat_dir_name(fd: i32, path: i32, path_len: i32) -> i32;

    /// Write to a file descriptor, without using and updating the file descriptor's offset.
    ///
    /// # Note
    ///
    /// This is similar to `pwritev` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: file descriptor
    /// - `ciov_buf`, `ciov_buf_len`: Used to create `ciovec`,
    ///                               which is the list of scatter/gather vectors from which to retrieve data.
    /// - `offset`: The offset within the file at which to write.
    /// - `offsset0`: The size of bytes written is written here.
    fn fd_pwrite(fd: i32, ciov_buf: i32, ciov_buf_len: i32, offset: i64, offset0: i32) -> i32;

    /// Read from a file descriptor. Note: This is similar to readv in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `iov_buf`, `iov_buf_len`: used to create iovec, which is the list of scatter/gather vectors in which to store data.
    /// - `offset`: The offset within the file at which to read.
    /// - `offsset0`: size of bytes read is written here
    fn fd_read(fd: i32, iov_buf: i32, iov_buf_len: i32, offset1: i32) -> i32;

    /// Read directory entries from a directory.
    ///
    /// # Note
    ///
    /// - When successful, the contents of the output buffer consist of a sequence of directory entries.
    /// - Each directory entry consists of a `dirent` object,
    ///   followed by `dirent::d_namlen` bytes holding the name of the directory entry.
    /// - This function fills the output buffer as much as possible,
    ///   potentially truncating the last directory entry.
    /// - This allows the caller to grow its read buffer size in case it's too small
    ///   to fit a single large directory entry, or skip the oversized directory entry.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `buf`: The buffer where directory entries are stored.
    /// - `buf_len`: The length of the `buf` buffer.
    /// - `cookie`: The location within the directory to start reading.
    /// - `offset0`: The result, i.e. the number of bytes stored in the read buffer, is stored at this offset in memory
    ///              if less than the size of the read buffer, the end of the directory has been reached.
    fn fd_readdir(fd: i32, buf: i32, buf_len: i32, cookie: i64, offset0: i32) -> i32;

    /// Atomically replace a file descriptor by renumbering another file descriptor.
    ///
    /// # Note
    ///
    /// - Due to the strong focus on thread safety, this environment does not provide a mechanism
    ///   to duplicate or renumber a file descriptor to an arbitrary number, like `dup2()`.
    ///   This would be prone to race conditions, as an actual file descriptor with the same number
    ///   could be allocated by a different thread at the same time.
    /// - This function provides a way to atomically renumber file descriptors,
    ///   which would disappear if `dup2()` were to be removed entirely.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `to`: The file descriptor to overwrite.
    fn fd_renumber(fd: i32, to: i32) -> i32;

    /// Move the offset of a file descriptor.
    ///
    /// # Note
    ///
    /// This is similar to `lseek` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `offset`: The number of bytes to move.
    /// - `whence`: The base from which the offset is relative
    /// - `offset0`: The memory location to which the new offset of the file descriptor,
    ///              relative to the start of the file is stored.
    fn fd_seek(fd: i32, offset: i64, whence: i32, offset0: i32) -> i32;

    /// Synchronize the data and metadata of a file to disk.
    ///
    /// # Note
    ///
    /// This is similar to `fsync` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    fn fd_sync(fd: i32) -> i32;

    /// Return the current offset of a file descriptor.
    ///
    /// # Note
    ///
    /// This is similar to `lseek(fd, 0, SEEK_CUR)` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `offset0`: Offset into the memory where result is stored upon success.
    /// - `result`: The current offset of the file descriptor, relative to the start of the file.
    fn fd_tell(fd: i32, offset0: i32) -> i32;

    /*
    /// Write to a file descriptor.
    ///
    /// # Note
    ///
    /// This is similar to `writev` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `ciov_buf`, `ciov_buf_len`: used to create ciovec, which is the list of
    ///                               scatter/gather vectors from which to retrieve data.
    /// - `offset0`: The offset into the memory where result (size written) is stored
    fn fd_write(fd: i32, ciov_buf: i32, ciov_buf_len: i32, offset0: i32) -> i32;*/

    /// Create a directory.
    ///
    /// # Note
    ///
    /// This is similar to `mkdirat` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `offset`, `length`: The offset/length pair used to create a guest pointer into host memory.
    ///                       This pointer references the path string at which to create the directory.
    fn path_create_directory(fd: i32, offset: i32, length: i32) -> i32;

    /// Return the attributes of a file or directory.
    ///
    /// # Note
    ///
    /// This is similar to `stat` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `flags`: Flags determining the method of how the path is resolved.
    /// - `offset`, `length`: The offset/length pair used to create a guest pointer into host memory.
    ///                       This pointer references the path string of the file or directory to inspect.
    /// - `offset0`: The buffer where the file's attributes are stored.
    fn path_filestat_get(fd: i32, flags: i32, offset: i32, length: i32, offset0: i32) -> i32;

    /// Adjust the timestamps of a file or directory.
    ///
    /// # Note
    ///
    /// This is similar to `utimensat` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `flags`: Flags determining the method of how the path is resolved.
    /// - `offset`, `length`: The offset/length pair used to create a guest pointer into host memory.
    ///                       This pointer references the path string of the file or directory to operate on.
    /// - `atim`: The desired values of the data access timestamp.
    /// - `mtim`: The desired values of the data modification timestamp.
    /// - `fst_flags`: A bitmask indicating which timestamps to adjust.
    fn path_filestat_set_times(
        fd: i32,
        flags: i32,
        offset: i32,
        length: i32,
        atim: i64,
        mtim: i64,
        fst_flags: i32,
    ) -> i32;

    /// Create a hard link.
    ///
    /// # Note
    ///
    /// This is similar to `linkat` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `old_fd`: file descriptor
    /// - `old_flags`: Flags determining the method of how the path is resolved.
    /// - `old_offset`, `old_length`: The offset/length pair used to create a guest pointer into host memory.
    ///                               This pointer references the path string source path from which to link.
    /// - `new_fd`: The working directory at which the resolution of the new path starts.
    /// - `new_offset`, `new_length`: The offset/length pair used to create a guest pointer into host memory.
    ///                               This pointer references the path string, i.e. ehe destination path at
    ///                               which to create the hard link.
    fn path_link(
        old_fd: i32,
        old_flags: i32,
        old_offset: i32,
        old_length: i32,
        new_fd: i32,
        new_offset: i32,
        new_length: i32,
    ) -> i32;

    /// Open a file or directory.
    ///
    /// # Note
    ///
    /// - The returned file descriptor is not guaranteed to be the lowest-numbered file descriptor not currently open;
    ///   it is randomized to prevent applications from depending on making assumptions about indexes,
    ///   since this is error-prone in multi-threaded contexts.
    /// - The returned file descriptor is guaranteed to be less than 2^31.
    /// - This is similar to `openat` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `dirflags`: Flags determining the method of how the path is resolved.
    /// - `offset`, `length`: The offset/length pair used to create a guest pointer into host memory.
    ///                       This pointer references the relative path of the file or directory to open,
    ///                       relative to the `path_open::fd` directory.
    /// - `oflags`: The method by which to open the file.
    /// - `fs_rights_base`: The initial rights of the newly created file descriptor
    /// - `fs_rights_inheriting`: The rights to inherit.
    /// - `fdflags`: The file descriptor flags.
    /// - `offset0`: The offset into memory where result is stored.
    ///              The result is the file descriptor of the file that has been opened.
    fn path_open(
        fd: i32,
        dirflags: i32,
        offset: i32,
        length: i32,
        oflags: i32,
        fs_rights_base: i64,
        fdflags: i64,
        fs_rights_inheriting: i32,
        offfset0: i32,
    ) -> i32;

    /// Read the contents of a symbolic link.
    ///
    /// # Note
    ///
    /// This is similar to `readlinkat` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `offset`, `length`: The offset/length pair used to create a guest pointer into host memory.
    ///                       This pointer references the path of the symbolic link from which to read.
    /// - `buf`: The buffer to which to write the contents of the symbolic link.
    /// - `buf_len`: The length of the `buf` buffer.
    /// - `offset0`: The offset into memory where result is stored.
    ///              The result is the number of bytes placed in the buffer.
    fn path_readlink(
        fd: i32,
        offset: i32,
        length: i32,
        buf: i32,
        buf_len: i32,
        offset0: i32,
    ) -> i32;

    /// Remove a directory.
    ///
    /// # Note
    ///
    /// - Returns `errno::notempty` if the directory is not empty.
    /// - This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `offset`, `length`: The offset/length pair used to create a guest pointer into host memory.
    ///                       This pointer references the path to the directory to remove.
    fn path_remove_directory(fd: i32, offset: i32, length: i32) -> i32;

    /// Rename a file or directory.
    ///
    /// # Note
    ///
    /// - This is similar to `renameat` in POSIX.
    /// - This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `old_offset`, `old_length`: The offset/length pair used to create a guest pointer into host memory.
    ///                               This pointer references the source path of the file or directory to rename.
    /// - `new_fd`: The working directory at which the resolution of the new path starts.
    /// - `new_offset`, `new_length`: The offset/length pair used to create a guest pointer into host memory.
    ///                               This pointer references the destination path to which to rename the file or directory.
    fn path_rename(
        fd: i32,
        old_offset: i32,
        old_length: i32,
        new_fd: i32,
        new_offset: i32,
        new_length: i32,
    ) -> i32;

    /// Create a symbolic link.
    ///
    /// # Note
    ///
    /// This is similar to `symlinkat` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `old_offset`, `old_length`: The offset/length pair used to create a guest pointer into host memory.
    ///                               This pointer references the path to the contents of the symbolic link.
    /// - `fd`: The file descriptor.
    /// - `new_offset`, `new_length`: The offset/length pair used to create a guest pointer into host memory.
    ///                               This pointer references the destination path at which to create the symbolic link.
    fn path_symlink(
        old_offset: i32,
        old_length: i32,
        fd: i32,
        new_offset: i32,
        new_length: i32,
    ) -> i32;

    /// Unlink a file.
    ///
    /// # Note
    ///
    /// - Returns `errno::isdir` if the path refers to a directory.
    /// - This is similar to `unlinkat(fd, path, 0)` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `offset`, `length`: The offset/length pair used to create a guest pointer into host memory.
    ///                       This pointer references the path to a file to unlink.
    fn path_unlink_file(fd: i32, offset: i32, length: i32) -> i32;

    /// Concurrently poll for the occurrence of a set of events.
    ///
    /// # Parameters
    ///
    /// - `in_`: The events to which to subscribe.
    /// - `out`: The events that have occurred.
    /// - `nsubscriptions`: Both the number of subscriptions and events.
    /// - `offset0`: The offset into memory where the number of events is stored.
    fn poll_oneoff(in_: i32, out: i32, nsubscriptions: i32, offset0: i32) -> i32;

    /// Send a signal to the process of the calling thread.
    /// Note: This is similar to `raise` in POSIX.
    /// # Parameters
    ///
    /// sig: The signal condition to trigger.
    fn proc_raise(sig: i32) -> i32;

    /// Accept a new incoming connection.
    ///
    /// # Note
    ///
    /// This is similar to `accept` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The listening socket.
    /// - `flags`: The desired values of the file descriptor flags.
    /// - `offset0`: The offset into memory where the new socket connection `fd` is stored.
    fn sock_accept(fd: i32, flags: i32, offset0: i32) -> i32;

    /// Receive a message from a socket.
    ///
    /// # Note
    ///
    /// This is similar to `recv` in POSIX, though it also supports reading
    /// the data into multiple buffers in the manner of `readv`.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `iov_buf`, `iov_buf_len`: Used to create `iovec`, which is the list of scatter/gather
    ///                             vectors in which to store data.
    /// - `ri_flags`: The message flags.
    /// - `offset0`, `offset1`: The offset into memory where the number of
    ///                         bytes in `ri_data` and message flags are stored.
    fn sock_recv(
        fd: i32,
        iov_buf: i32,
        iov_buf_len: i32,
        ri_flags: i32,
        offset0: i32,
        offset1: i32,
    ) -> i32;

    /// Send a message on a socket.
    ///
    /// # Note
    ///
    /// This is similar to `send` in POSIX, though it also supports writing
    /// the data from multiple buffers in the manner of `writev`.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `ciov_buf`, `ciov_buf_len`: Used to create ciovec, which is the list of
    ///                               scatter/gather vectors from which to retrieve data.
    /// - `si_flags`: The message flags.
    /// - `offset0`: The offset into memory where number of bytes transmitted is stored.
    fn sock_send(fd: i32, ciov_buf: i32, ciov_buf_len: i32, si_flags: i32, offset0: i32) -> i32;

    /// Shut down socket send and receive channels.
    ///
    /// # Note
    ///
    /// This is similar to `shutdown` in POSIX.
    ///
    /// # Parameters
    ///
    /// - `fd`: The file descriptor.
    /// - `how`: Which channels on the socket to shut down.
    fn sock_shutdown(fd: i32, how: i32) -> i32;
);