--[[
THIS DOES NOT WORK RIGHT NOW.
VOXEL SUPPORT IS STILL EXTREMELY WIP, NOTHING HERE IS FINAL.
Thanks :)
]]--

local winSizeX = 1
local winSizeY = 1

-- local shader = husky.graphics:newShader("voxel_raytrace.glsl")

local scene = husky.voxel:newScene()

function husky.draw()
	husky.graphics:clear(0.5, 0.2, 0.35)

	winSizeX, winSizeY = husky.graphics:getSize()

	husky.graphics:setColor(1.0, 1.0, 1.0)
	-- husky.graphics:setShader(shader)
    husky.graphics:rect("fill", 0,0, winSizeX, winSizeY)
end
