local winSizeX = 1
local winSizeY = 1

local shader = husky.graphics:newShader("test_fs.glsl")

function husky.draw()
	husky.graphics:clear(0.5, 0.2, 0.35)

	winSizeX, winSizeY = husky.graphics:getSize()

	local size = winSizeX
	if winSizeY < size then size = winSizeY end
	size = size / 1.5

	husky.graphics:setColor(0.75, 0.45, 0.3)
	husky.graphics:rect("fill", winSizeX / 4 - size / 2, winSizeY / 2 - size / 2, size, size)
	husky.graphics:setShader(shader)
	shader:uniform("grayness", 1)
	husky.graphics:rect("fill", winSizeX / 4 * 3 - size / 2, winSizeY / 2 - size / 2, size, size)
	shader:uniform("grayness", 0)
	husky.graphics:rect("fill", winSizeX / 4 * 3 - size / 4, winSizeY / 2 - size / 4, size / 2, size / 2)
end
