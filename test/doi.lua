local M = {}

local DOI_PATTERN = "%f[%w]10%.%d%d%d%d+/[^%s&\"']*[^%s&\"'.,]"



--- Split the URL on ? or # so fragments or queries don't corrupt the DOI match
---@param url string The URL to clean
local function cleanURL(url)
  local parts = {}
  local url = url:gsub("^https?://", "")
  for part in url:gmatch("([^%?#]+)") do
    parts[#parts + 1] = part
  end
  return parts
end

--- Parse an instance of a DOI url
---@param url string DOI URL
---@return string | nil The DOI code
function M.parse(url)
  for _, part in ipairs(cleanURL(url)) do
    local doi = part:match(DOI_PATTERN)
    if doi then
      return doi
    end
  end
  return nil
end

local function dump(o)
  if type(o) == 'table' then
    local s = '{ '
    for k, v in pairs(o) do
      if type(k) ~= 'number' then k = '"' .. k .. '"' end
      s = s .. '[' .. k .. '] = ' .. dump(v) .. ','
    end
    return s .. '} '
  else
    return tostring(o)
  end
end

function M.fetch(doi)
  -- 1. Build the API URL
  local url = "https://api.crossref.org/works/" .. doi

  -- 2. Delegate HTTP & JSON to the Rust function
  local data = fetch(url)
  if not data then
    error("Fetch failed for DOI: " .. doi)
  end

  -- Extract and reshape fields for BibFetch
  local result = {
    doi       = doi,
    title     = data.title,
    authors   = data.author or {},
    journal   = (data["container-title"] and data["container-title"][1]) or nil,
    year      = (data.published and data.published["date-parts"]
      and data.published["date-parts"][1][1]) or nil,
    publisher = data.publisher,
    url       = data.URL,
  }
  print(dump(result))
  return result
end

return M
