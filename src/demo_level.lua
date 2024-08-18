print( '\nentered Lua..' )


function update(time)
	return '- + -'
end


function setup(params)
	print( 'lua.setup..' )
	print( 'X = ', params["X"], 'Y = ', params["Y"] )
end

