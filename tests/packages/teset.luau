--!nocheck

local WinLuxIO = require("@lux_packages/winluxio")

local Theme = WinLuxIO.Theme
local Window = WinLuxIO.Window
local Frame = WinLuxIO.Frame
local Card = WinLuxIO.Card
local Label = WinLuxIO.Label
local TextInput = WinLuxIO.TextInput
local Button = WinLuxIO.Button
local Switch = WinLuxIO.Switch

Theme.setDark(true)

local win = Window.new({
	Title = "WinLuxIO Login",
	Width = 700,
	Height = 500,
	Centered = true,
})

local background = Frame.new({
	Parent = win,
	Size = UDim2.fromScale(1, 1),
	BackgroundColor = Theme.Surface,
	BackgroundTransparency = 0,
})

local topBar = Frame.new({
	Parent = background,
	Size = UDim2.new(1, 0, 0, 80),
	BackgroundTransparency = 1,
})

Label.new({
	Parent = topBar,
	Text = "WinLuxIO",
	Position = UDim2.fromOffset(40, 20),
	Size = UDim2.fromOffset(200, 40),
	TextColor = Theme.Primary,
	TextSize = 24,
	Font = "Segoe UI Bold",
	BackgroundTransparency = 1,
})

local toggleContainer = Frame.new({
	Parent = topBar,
	Size = UDim2.fromOffset(160, 40),
	Position = UDim2.new(1, -200, 0, 20),
	BackgroundTransparency = 1,
})

Label.new({
	Parent = toggleContainer,
	Text = "Dark Mode",
	Position = UDim2.fromOffset(0, 8),
	Size = UDim2.fromOffset(80, 24),
	TextColor = Theme.OnSurface,
	BackgroundTransparency = 1,
	TextAlign = "right",
})

local themeSwitch = Switch.new({
	Parent = toggleContainer,
	Position = UDim2.fromOffset(90, 4),
	Mode = "auto",
	On = true,
})

local loginCard = Card.new({
	Parent = background,
	Size = UDim2.fromOffset(420, 580),
	Position = UDim2.fromScale(0.5, 0.5),
	AnchorPoint = Vector2.new(0.5, 0.5),
	Style = "elevated",
	CornerRadius = 24,
})

local contentPadding = 40
local contentWidth = 420 - (contentPadding * 2)

Label.new({
	Parent = loginCard,
	Text = "Welcome Back",
	Position = UDim2.fromOffset(contentPadding, 48),
	Size = UDim2.fromOffset(contentWidth, 40),
	TextColor = Theme.OnSurface,
	TextSize = 32,
	TextAlign = "center",
	BackgroundTransparency = 1,
	Font = "Segoe UI Semibold",
})

Label.new({
	Parent = loginCard,
	Text = "Enter your credentials to access your account",
	Position = UDim2.fromOffset(contentPadding, 92),
	Size = UDim2.fromOffset(contentWidth, 20),
	TextColor = Theme.OnSurfaceVariant,
	TextSize = 14,
	TextAlign = "center",
	BackgroundTransparency = 1,
})

local emailInput = TextInput.new({
	Parent = loginCard,
	PlaceholderText = "name@example.com",
	Position = UDim2.fromOffset(contentPadding, 150),
	Size = UDim2.fromOffset(contentWidth, 50),
	CornerRadius = 12,
	BackgroundColor = Theme.SurfaceContainerHighest,
})

local passInput = TextInput.new({
	Parent = loginCard,
	PlaceholderText = "Password",
	Position = UDim2.fromOffset(contentPadding, 216),
	Size = UDim2.fromOffset(contentWidth, 50),
	CornerRadius = 12,
	Secure = true,
	BackgroundColor = Theme.SurfaceContainerHighest,
})

Label.new({
	Parent = loginCard,
	Text = "Forgot Password?",
	Position = UDim2.fromOffset(contentPadding, 276),
	Size = UDim2.fromOffset(contentWidth, 20),
	TextColor = Theme.Primary,
	TextSize = 13,
	TextAlign = "right",
	BackgroundTransparency = 1,
	Font = "Segoe UI Semibold",
})

local loginBtn = Button.new({
	Parent = loginCard,
	Text = "Sign In",
	Position = UDim2.fromOffset(contentPadding, 320),
	Size = UDim2.fromOffset(contentWidth, 50),
	Style = "filled",
})

loginBtn.MouseButton1Click:Connect(function()
	loginBtn.Text = "Authenticating..."
	task.wait(1)
	loginBtn.Text = "Sign In"
end)

emailInput.TextChanged:Connect(function(text)
	print("Email:", text)
end)

passInput.SubmitPressed:Connect(function(text)
	print("Password submitted, length:", #text)
end)

Label.new({
	Parent = loginCard,
	Text = "or continue with",
	Position = UDim2.fromOffset(contentPadding, 400),
	Size = UDim2.fromOffset(contentWidth, 20),
	TextColor = Theme.Outline,
	TextSize = 12,
	TextAlign = "center",
	BackgroundTransparency = 1,
})

local socialWidth = (contentWidth - 16) / 2

local googleBtn = Button.new({
	Parent = loginCard,
	Text = "Google",
	Position = UDim2.fromOffset(contentPadding, 440),
	Size = UDim2.fromOffset(socialWidth, 44),
	Style = "tonal",
	TextColor = Theme.OnSurface,
})

local githubBtn = Button.new({
	Parent = loginCard,
	Text = "GitHub",
	Position = UDim2.fromOffset(contentPadding + socialWidth + 16, 440),
	Size = UDim2.fromOffset(socialWidth, 44),
	Style = "tonal",
	TextColor = Theme.OnSurface,
})

Label.new({
	Parent = background,
	Text = "Don't have an account? Sign up",
	Position = UDim2.new(0.5, -150, 1, -60),
	Size = UDim2.fromOffset(300, 30),
	TextColor = Theme.OnSurfaceVariant,
	TextAlign = "center",
	BackgroundTransparency = 1,
})

win:show(false)
WinLuxIO.run()
