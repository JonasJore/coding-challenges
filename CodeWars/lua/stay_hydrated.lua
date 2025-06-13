local litres = function(time)
    local litres_drank = 0
    for _ = 1, time, 1 do
        litres_drank = litres_drank + 0.5
    end
    return math.floor(litres_drank)
end

return litres
