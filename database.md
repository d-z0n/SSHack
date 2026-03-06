

## Table: users
|  id  (primary)  |  name  |  hashed_password  |
|-----------------|--------|-------------------|
|        1        |  Dz0N  |  ................ |

## Table: flags
|  id (primary)  |  name |   description (md)   |  points  |  flag  |
|----------------|-------|----------------------|----------|--------|
|       1        | flag0 | [an easy flag](link) |  100     |ctf{...}|

## Table: cleared
|  id (primary)  |  uid (foreign) |  fid (foreign)  |
|----------------|----------------|-----------------|
|       1        |     1 (Dz0N)   |    1 (flag0)    |

