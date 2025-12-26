-- Test Enum
print("[TEST] Enum")

-- Enum global should exist
assert(Enum ~= nil, "Enum global should exist")

-- KeyCode
assert(Enum.KeyCode ~= nil, "Enum.KeyCode should exist")
assert(Enum.KeyCode.A == 0x41, "KeyCode.A should be 0x41 (Windows VK)")
assert(Enum.KeyCode.Space == 0x20, "KeyCode.Space should be 0x20")
assert(Enum.KeyCode.Return == 0x0D, "KeyCode.Return should be 0x0D")
assert(Enum.KeyCode.Escape == 0x1B, "KeyCode.Escape should be 0x1B")
assert(Enum.KeyCode.F1 == 0x70, "KeyCode.F1 should be 0x70")
assert(Enum.KeyCode.Up == 0x26, "KeyCode.Up should be 0x26")
assert(Enum.KeyCode.LeftShift == 0xA0, "KeyCode.LeftShift should be 0xA0")

-- MouseButton
assert(Enum.MouseButton ~= nil, "Enum.MouseButton should exist")
assert(Enum.MouseButton.Left == 0, "MouseButton.Left should be 0")
assert(Enum.MouseButton.Right == 1, "MouseButton.Right should be 1")
assert(Enum.MouseButton.Middle == 2, "MouseButton.Middle should be 2")

-- EasingStyle
assert(Enum.EasingStyle ~= nil, "Enum.EasingStyle should exist")
assert(Enum.EasingStyle.Linear == 0, "EasingStyle.Linear should be 0")
assert(Enum.EasingStyle.Quad == 1, "EasingStyle.Quad should be 1")
assert(Enum.EasingStyle.Bounce == 10, "EasingStyle.Bounce should be 10")

-- EasingDirection
assert(Enum.EasingDirection ~= nil, "Enum.EasingDirection should exist")
assert(Enum.EasingDirection.In == 0, "EasingDirection.In should be 0")
assert(Enum.EasingDirection.Out == 1, "EasingDirection.Out should be 1")
assert(Enum.EasingDirection.InOut == 2, "EasingDirection.InOut should be 2")

-- SortOrder
assert(Enum.SortOrder ~= nil, "Enum.SortOrder should exist")
assert(Enum.SortOrder.LayoutOrder == 0, "SortOrder.LayoutOrder should be 0")
assert(Enum.SortOrder.Name == 1, "SortOrder.Name should be 1")

print("[PASS] Enum")
