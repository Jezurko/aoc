input = readchomp( "input.txt" )
commands = split( input, "\n" )

function cd( dir, pwd )
    if dir == "/"
        pwd = ""
        return pwd
    end
    if dir == ".."
        pos = findlast( '/', pwd )
        if pos == nothing
            pos = 0
        end
        pwd = pwd[begin:pos - 1]
        return pwd
    end
    pwd = pwd * "/" * dir
    return pwd
end

function file( size, pwd, folders )
    while( pwd != "" )
        new_size = size + get( folders, pwd, 0 )
        folders[pwd] = new_size
        pwd = pwd[begin:findlast( '/', pwd ) - 1]
    end
    new_size = size + get( folders, pwd, 0 )
    folders[pwd] = new_size
    return folders
end

function execute_lines( commands )
    folder_sizes = Dict( "" => 0 )
    pwd = ""
    for command in commands
        if startswith( command, "\$ cd" )
            pwd = cd( split( command, " " )[end], pwd )
        end
        if occursin( r"^[0-9]+ .*", command )
            folder_sizes = file( parse( Int, split( command, " ")[begin] ), pwd, folder_sizes )
        end
    end
    return folder_sizes
end

find_sub_10k( folders ) = sum( values( filter( x->(last(x) < 100000 ), folders ) ) )

function find_candidate( folders )
    free_space = 70000000 - folders[""]
    min( values( filter( x->( (last(x) + free_space) > 30000000 ), folders ) )... )
end

println( find_sub_10k( execute_lines( commands ) ) )

println( find_candidate( execute_lines( commands ) ) )
