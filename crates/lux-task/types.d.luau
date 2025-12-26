--!nocheck
--[=[
    @class task
    Coroutine-based task scheduling for concurrent programming.
    
    The task library provides Roblox-compatible task scheduling, allowing you to
    run multiple operations concurrently without blocking the main thread.
    
    ## Spawning Tasks
    ```lua
    -- Immediately run a function in a new thread
    local thread = task.spawn(function()
        print("Running immediately!")
    end)
    
    -- Pass arguments to the spawned function
    task.spawn(function(name, count)
        print(name, count)  -- "Hello", 42
    end, "Hello", 42)
    ```
    
    ## Deferring Tasks
    ```lua
    -- Run after current thread yields (end of current "frame")
    task.defer(function()
        print("Deferred execution")
    end)
    
    print("This prints first")
    -- Output:
    -- This prints first
    -- Deferred execution
    ```
    
    ## Delayed Execution
    ```lua
    -- Execute after a delay
    task.delay(2, function()
        print("2 seconds later!")
    end)
    
    -- With arguments
    task.delay(1.5, function(msg)
        print(msg)
    end, "Hello after 1.5s")
    ```
    
    ## Waiting
    ```lua
    -- Pause execution for a duration
    local elapsed = task.wait(1.0)  -- Wait 1 second
    print("Waited for", elapsed, "seconds")
    
    -- Minimum wait (yields then resumes quickly)
    task.wait()  -- Yields control, resumes next cycle
    ```
    
    ## Cancellation
    ```lua
    local thread = task.delay(10, function()
        print("This won't run")
    end)
    
    -- Cancel before it executes
    task.cancel(thread)
    ```
    
    ## Example: Parallel Downloads
    ```lua
    local function downloadFile(url)
        task.spawn(function()
            local data = net.get(url)
            print("Downloaded:", url)
        end)
    end
    
    -- Start multiple downloads concurrently
    downloadFile("https://example.com/file1")
    downloadFile("https://example.com/file2")
    downloadFile("https://example.com/file3")
    
    -- All downloads run in parallel
    ```
]=]
export type task = {
    --- Immediately spawns a new thread to run the function
    --- @param func function -- The function to execute
    --- @param ... any -- Arguments to pass to the function
    --- @return thread -- The thread handle
    spawn: <T...>(func: (T...) -> (), ...: T...) -> thread,
    
    --- Defers execution until the current thread yields
    --- @param func function -- The function to execute
    --- @param ... any -- Arguments to pass to the function
    --- @return thread -- The thread handle
    defer: <T...>(func: (T...) -> (), ...: T...) -> thread,
    
    --- Schedules a function to execute after a delay
    --- @param seconds number -- Delay in seconds
    --- @param func function -- The function to execute
    --- @param ... any -- Arguments to pass to the function
    --- @return thread -- The thread handle
    delay: <T...>(seconds: number, func: (T...) -> (), ...: T...) -> thread,
    
    --- Pauses the current thread for a duration
    --- @param seconds number? -- Duration to wait (default: minimum yield)
    --- @return number -- Actual time elapsed
    wait: (seconds: number?) -> number,
    
    --- Cancels a scheduled or running thread
    --- @param thread thread -- The thread to cancel
    cancel: (thread: thread) -> (),
}
return {} :: task
