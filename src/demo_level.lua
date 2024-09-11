print( 'entered Lua..' )


function update(time)
	-- print("time:", time)
	local target = { x=9, y=5 }
	local obstacles = { {x=15,y=5}, {x=16,y=5}, {x=17,y=4} }

	return {
		target=target,
		obstacles=obstacles
	}
end

