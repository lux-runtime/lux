--!nocheck

--[=[
    @class Signal

    High-performance event system for Lux (Roblox-style API).

    Signals allow you to create custom events that can have multiple listeners.
    When fired, all connected callbacks are executed with the provided arguments.

    ### Features
    - Multiple listeners per signal
    - One-time listeners with `Once()`
    - Wait for next fire with `Wait()`
    - Disconnect individual listeners or all at once
    - Type-safe with generics

    ### Example usage

    ```lua
    local Signal = require("@lux/signal")

    -- Create a signal
    local onPlayerJoined = Signal.new()

    -- Connect a listener
    local connection = onPlayerJoined:Connect(function(playerName, score)
        print(playerName .. " joined with score " .. score)
    end)

    -- Fire the signal
    onPlayerJoined:Fire("Alice", 100)  -- Prints: Alice joined with score 100
    onPlayerJoined:Fire("Bob", 250)    -- Prints: Bob joined with score 250

    -- Disconnect when done
    connection:Disconnect()

    -- One-time listener
    onPlayerJoined:Once(function(name)
        print("First player after reset: " .. name)
    end)
    
    -- Wait for signal in a coroutine
    task.spawn(function()
        local name, score = onPlayerJoined:Wait()
        print("Waited for:", name, score)
    end)
    ```
]=]

-- ============================================================================
-- Types
-- ============================================================================

--[=[
    @interface Connection
    @within Signal

    Represents a connection between a Signal and a callback function.

    Properties:
    * `Connected` - Whether the connection is still active

    Methods:
    * `Disconnect()` - Removes this connection, callback will no longer be called

    ### Example
    ```lua
    local signal = Signal.new()
    
    local conn = signal:Connect(function()
        print("Called!")
    end)
    
    print(conn.Connected) -- true
    signal:Fire() -- Prints: Called!
    
    conn:Disconnect()
    print(conn.Connected) -- false
    signal:Fire() -- Nothing happens
    ```
]=]
export type Connection = {
	Connected: boolean,
	Disconnect: (self: Connection) -> (),
}

--[=[
    @interface Signal
    @within Signal

    A signal that can have multiple listeners and fire events.

    Generic type `T...` represents the argument types that will be passed to callbacks.

    ### Methods
    
    | Method | Description |
    |--------|-------------|
    | `Connect(callback)` | Add a persistent listener |
    | `Once(callback)` | Add a one-time listener |
    | `Fire(...)` | Trigger all callbacks |
    | `Wait()` | Yield until next fire |
    | `DisconnectAll()` | Remove all listeners |
    | `Destroy()` | Clean up the signal |
    | `GetConnections()` | Count active listeners |

    ### Example
    ```lua
    -- Typed signal for damage events
    local onDamage = Signal.new() -- Signal<number, string, boolean>
    
    onDamage:Connect(function(amount, source, isCritical)
        local msg = source .. " dealt " .. amount .. " damage"
        if isCritical then msg = msg .. " (CRITICAL!)" end
        print(msg)
    end)
    
    onDamage:Fire(50, "Enemy", false)   -- Enemy dealt 50 damage
    onDamage:Fire(150, "Boss", true)    -- Boss dealt 150 damage (CRITICAL!)
    ```
]=]
export type Signal<T...> = {
	--[=[
        @within Signal
        
        Connects a callback function to the signal.
        The callback will be called every time the signal fires.
        
        @param callback -- Function to call when signal fires
        @return Connection -- A connection object to manage this listener
        
        ### Example
        ```lua
        local onUpdate = Signal.new()
        
        local conn = onUpdate:Connect(function(dt)
            print("Delta time:", dt)
        end)
        
        onUpdate:Fire(0.016)  -- Delta time: 0.016
        onUpdate:Fire(0.017)  -- Delta time: 0.017
        
        conn:Disconnect()
        onUpdate:Fire(0.016)  -- Nothing (disconnected)
        ```
    ]=]
	Connect: (self: Signal<T...>, callback: (T...) -> ()) -> Connection,

	--[=[
        @within Signal
        
        Connects a callback that will only be called once, then auto-disconnects.
        
        @param callback -- Function to call when signal fires
        @return Connection -- A connection object (will auto-disconnect after first fire)
        
        ### Example
        ```lua
        local onReady = Signal.new()
        
        onReady:Once(function()
            print("System initialized!")
        end)
        
        onReady:Fire() -- System initialized!
        onReady:Fire() -- Nothing (auto-disconnected)
        ```
    ]=]
	Once: (self: Signal<T...>, callback: (T...) -> ()) -> Connection,

	--[=[
        @within Signal
        
        Fires the signal, calling all connected callbacks with the provided arguments.
        
        @param ... -- Arguments to pass to all callbacks
        
        ### Example
        ```lua
        local onScore = Signal.new()
        
        onScore:Connect(function(player, points)
            print(player .. " scored " .. points .. " points!")
        end)
        
        onScore:Fire("Alice", 100)  -- Alice scored 100 points!
        onScore:Fire("Bob", 50)     -- Bob scored 50 points!
        ```
    ]=]
	Fire: (self: Signal<T...>, T...) -> (),

	--[=[
        @within Signal
        @yields
        
        Yields the current thread until the signal is fired.
        Returns the arguments that were passed to Fire().
        
        @return T... -- The arguments that were passed to Fire()
        
        ### Example
        ```lua
        local onComplete = Signal.new()
        
        task.spawn(function()
            print("Waiting for completion...")
            local result, success = onComplete:Wait()
            print("Completed with:", result, success)
        end)
        
        task.wait(1)
        onComplete:Fire("Done!", true)
        -- Output:
        -- Waiting for completion...
        -- (after 1 second)
        -- Completed with: Done! true
        ```
    ]=]
	Wait: (self: Signal<T...>) -> T...,

	--[=[
        @within Signal
        
        Disconnects all listeners from this signal.
        
        ### Example
        ```lua
        local signal = Signal.new()
        
        signal:Connect(function() print("A") end)
        signal:Connect(function() print("B") end)
        signal:Connect(function() print("C") end)
        
        signal:Fire() -- A, B, C
        
        signal:DisconnectAll()
        signal:Fire() -- Nothing
        ```
    ]=]
	DisconnectAll: (self: Signal<T...>) -> (),

	--[=[
        @within Signal
        
        Destroys the signal and disconnects all listeners.
        Same as DisconnectAll, but indicates the signal won't be reused.
        
        ### Example
        ```lua
        local widget = { Clicked = Signal.new() }
        
        widget.Clicked:Connect(function()
            print("Clicked!")
        end)
        
        -- When destroying the widget
        widget.Clicked:Destroy()
        widget.Clicked = nil
        ```
    ]=]
	Destroy: (self: Signal<T...>) -> (),

	--[=[
        @within Signal
        
        Returns the current number of active connections.
        
        @return number -- Number of connected listeners
        
        ### Example
        ```lua
        local signal = Signal.new()
        print(signal:GetConnections()) -- 0
        
        local c1 = signal:Connect(function() end)
        local c2 = signal:Connect(function() end)
        print(signal:GetConnections()) -- 2
        
        c1:Disconnect()
        print(signal:GetConnections()) -- 1
        ```
    ]=]
	GetConnections: (self: Signal<T...>) -> number,
}

-- ============================================================================
-- Module
-- ============================================================================

--[=[
    @within Signal

    Creates a new Signal instance.

    @return Signal<T...> -- A new signal that can fire events of type T...
    
    ### Example
    ```lua
    local Signal = require("@lux/signal")
    
    -- Signal with no arguments
    local onReset = Signal.new()
    onReset:Connect(function()
        print("Reset!")
    end)
    onReset:Fire()
    
    -- Signal with typed arguments (comment shows intended types)
    -- Signal<string, number>
    local onMessage = Signal.new()
    onMessage:Connect(function(sender, code)
        print(sender, "sent code", code)
    end)
    onMessage:Fire("Server", 200)
    
    -- Common patterns
    local Destroying = Signal.new()   -- Lifecycle
    local ValueChanged = Signal.new() -- Property changes (oldValue, newValue)
    local ChildAdded = Signal.new()   -- Hierarchy changes (child)
    ```
]=]
return {
	new = function<T...>(): Signal<T...>
		return {} :: any
	end,
}
