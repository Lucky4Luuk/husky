local winSizeX = 1
local winSizeY = 1

function husky.draw()
	husky.graphics:clear(0.5, 0.2, 0.35)

	winSizeX, winSizeY = husky.graphics:getSize()

	local size = winSizeX
	if winSizeY < size then size = winSizeY end
	size = size / 1.5

	husky.graphics:setColor(0.75, 0.45, 0.3)
	husky.graphics:rect("fill", winSizeX / 2 - size / 2, winSizeY / 2 - size / 2, size, size)
	husky.graphics:setColor(0.2, 0.8, 0.6)
	husky.graphics:circle("fill", winSizeX / 2, winSizeY / 2, size / 2)
	husky.graphics:setColor(0.5, 0.2, 0.8)
	husky.graphics:tri("fill", winSizeX / 2, winSizeY / 2, size, size)
end
