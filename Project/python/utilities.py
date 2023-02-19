
def parse_json_raw_to_arr(json_raw):
    new_arr = []

    global_counter = 0
    str_arr = list(json_raw["data"])
    
    temp_row = []
    while(global_counter < len(str_arr)):
        next_counter = str_arr[global_counter]
        if next_counter == '\n':
            new_arr.append(temp_row)
            temp_row = []
        else:
            temp_row.append(next_counter)
        
        global_counter += 1

    return new_arr