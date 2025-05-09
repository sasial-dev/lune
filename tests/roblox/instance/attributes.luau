local fs = require("@lune/fs")
local roblox = require("@lune/roblox") :: any

local BrickColor = roblox.BrickColor
local CFrame = roblox.CFrame
local Color3 = roblox.Color3
local ColorSequence = roblox.ColorSequence
local ColorSequenceKeypoint = roblox.ColorSequenceKeypoint
local Font = roblox.Font
local NumberRange = roblox.NumberRange
local NumberSequence = roblox.NumberSequence
local NumberSequenceKeypoint = roblox.NumberSequenceKeypoint
local Rect = roblox.Rect
local UDim = roblox.UDim
local UDim2 = roblox.UDim2
local Vector2 = roblox.Vector2
local Vector3 = roblox.Vector3
local Instance = roblox.Instance
local Enum = roblox.Enum

local modelFile = fs.readFile("tests/roblox/rbx-test-files/models/attributes/binary.rbxm")
local model = roblox.deserializeModel(modelFile)[1]

model:SetAttribute("Foo", "Bar")
model:SetAttribute("CFrame", CFrame.identity)
model:SetAttribute("Font", Font.new("Arial"))

local ATTRS_ACTUAL = model:GetAttributes()
local ATTRS_EXPECTED: { [string]: any } = {
	-- From the file
	Boolean = true,
	BrickColor = BrickColor.new("Really red"),
	Color3 = Color3.fromRGB(162, 0, 255),
	ColorSequence = ColorSequence.new({
		ColorSequenceKeypoint.new(0, Color3.new(1, 0, 0)),
		ColorSequenceKeypoint.new(0.5, Color3.new(0, 1, 0)),
		ColorSequenceKeypoint.new(1, Color3.new(0, 0, 1)),
	}),
	Number = 12345,
	NumberRange = NumberRange.new(5, 10),
	NumberSequence = NumberSequence.new({
		NumberSequenceKeypoint.new(0, 1),
		NumberSequenceKeypoint.new(0.5, 0),
		NumberSequenceKeypoint.new(1, 1),
	}),
	Rect = Rect.new(1, 2, 3, 4),
	String = "Hello, world!",
	UDim = UDim.new(0.5, 100),
	UDim2 = UDim2.new(0.5, 10, 0.7, 30),
	Vector2 = Vector2.new(10, 50),
	Vector3 = Vector3.new(1, 2, 3),
	Infinity = math.huge,
	NaN = 0 / 0,
	-- Extras we set
	Foo = "Bar",
	CFrame = CFrame.identity,
	Font = Font.new("Arial"),
}

for name, value in ATTRS_EXPECTED do
	local actual = ATTRS_ACTUAL[name]
	if actual ~= value then
		if value ~= value and actual ~= actual then
			continue -- NaN
		end
		error(
			string.format(
				"Expected attribute '%s' to have value '%s', got value '%s'",
				name,
				tostring(value),
				tostring(actual)
			)
		)
	end
end

-- Cloning instances should also clone attributes

local cloned = model:Clone()
ATTRS_ACTUAL = cloned:GetAttributes()

for name, value in ATTRS_EXPECTED do
	local actual = ATTRS_ACTUAL[name]
	if actual ~= value then
		if value ~= value and actual ~= actual then
			continue -- NaN
		end
		error(
			string.format(
				"Expected cloned attribute '%s' to have value '%s', got value '%s'",
				name,
				tostring(value),
				tostring(actual)
			)
		)
	end
end

-- Setting attributes on a new empty instance should work

local folder = Instance.new("Folder")
folder:SetAttribute("Foo", "Bar")
assert(folder:GetAttribute("Foo") == "Bar")

-- Setting attributes to nil should work

folder:SetAttribute("Foo", nil)
assert(folder:GetAttribute("Foo") == nil)

-- Writing files with modified attributes should work

local game = Instance.new("DataModel")
model.Parent = game

local placeFile = roblox.serializePlace(game)
fs.writeDir("bin/roblox")
fs.writeFile("bin/roblox/attributes.rbxl", placeFile)

local enum_attr = Instance.new("Folder")
enum_attr:SetAttribute("Foo", Enum.NormalId.Front)
assert(enum_attr:GetAttribute("Foo") == Enum.NormalId.Front)

local enum_attr_ser = roblox.serializeModel({ enum_attr })
local enum_attr_de = roblox.deserializeModel(enum_attr_ser)

assert(enum_attr_de[1]:GetAttribute("Foo") == Enum.NormalId.Front)
