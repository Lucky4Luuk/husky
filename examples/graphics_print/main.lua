function husky.draw()
	husky.graphics:clear(0.5, 0.2, 0.35);
	for i=0,40 do
		husky.graphics:print("text print " .. tostring(i), 0, i * 20);
	end
end
