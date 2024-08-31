print( 'entered Lua..' )


function update(time)
	-- print("time:", time)
	local target = { x=9, y=5 }
	local obstacles = {}

	return {
		target=target,
		obstacles=obstacles
	}
end

