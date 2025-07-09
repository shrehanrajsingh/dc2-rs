# File Transfer Protocol for dc2-rs

## Protocol Overview

This document describes the file transfer protocols used in dc2-rs.

## Client File Upload Protocol

The following sequence defines how clients upload files:

| Step | Data Type | Description |
|------|-----------|-------------|
| 1    | `u32`     | Filename length in bytes |
| 2    | `u8[]`    | Filename as byte array (UTF-8 encoded) |
| 3    | `u64`     | Total file size in bytes |
| 4    | `u32`     | Chunk size in bytes |
| 5    | -         | **For each chunk in sequence:** |
|      | `u32`     | &nbsp;&nbsp;• Chunk index (0-based) |
|      | `u32`     | &nbsp;&nbsp;• Data length in bytes |
|      | `u8[]`    | &nbsp;&nbsp;• Chunk data |
|      | `[u8; 32]`| &nbsp;&nbsp;• SHA-256 hash of chunk data |
| 6    | `u32`     | Termination marker (`u32::MAX`) |

### Upload Protocol Notes

- SHA-256 hashes are used for integrity verification of each chunk
- Chunks must be transmitted in sequential order (0, 1, 2, ...)
- The termination marker (`u32::MAX`) signals the completion of file transfer

## Data Storage Protocol

The system uses the following directory structure for file operations:

- `/dump` - Destination directory for all received assets
- `/hostfile` - Source directory for all client uploads
<br><br>
> **Note**: This protocol specification is currently a proposed standard. It will be formally adopted following the first stable implementation release.