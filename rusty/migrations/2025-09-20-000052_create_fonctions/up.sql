-- Your SQL goes here

CREATE FUNCTION public.gold_bonus_for_fortress(fid integer)
RETURNS integer
LANGUAGE sql
AS $$
  SELECT COALESCE(SUM(level) FILTER (WHERE name = 'bank'), 0)
  FROM buildings
  WHERE fortress_id = fid;
$$;

CREATE FUNCTION public.food_bonus_for_fortress(fid integer)
RETURNS integer
LANGUAGE sql
AS $$
  SELECT COALESCE(SUM(level) FILTER (WHERE name = 'farm'), 0)
  FROM buildings
  WHERE fortress_id = fid;
$$;

CREATE FUNCTION public.wood_bonus_for_fortress(fid integer)
RETURNS integer
LANGUAGE sql
AS $$
  SELECT COALESCE(SUM(level) FILTER (WHERE name = 'sawmill'), 0)
  FROM buildings
  WHERE fortress_id = fid;
$$;

CREATE FUNCTION public.energy_bonus_for_fortress(fid integer)
RETURNS integer
LANGUAGE sql
AS $$
  SELECT COALESCE(SUM(level) FILTER (WHERE name = 'sanctuary'), 0)
  FROM buildings
  WHERE fortress_id = fid;
$$;
