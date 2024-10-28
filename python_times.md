### Candy Data Processing
#### Input Data:
```
                competitorname  chocolate  fruity  caramel  peanutyalmondy  nougat  ...  hard  bar  pluribus  sugarpercent  pricepercent  winpercent
0                    100 Grand          1       0        1               0       0  ...     0    1         0         0.732         0.860   66.971725
1                 3 Musketeers          1       0        0               0       1  ...     0    1         0         0.604         0.511   67.602936
2                     One dime          0       0        0               0       0  ...     0    0         0         0.011         0.116   32.261086
3                  One quarter          0       0        0               0       0  ...     0    0         0         0.011         0.511   46.116505
4                    Air Heads          0       1        0               0       0  ...     0    0         0         0.906         0.511   52.341465
..                         ...        ...     ...      ...             ...     ...  ...   ...  ...       ...           ...           ...         ...
80                   Twizzlers          0       1        0               0       0  ...     0    0         0         0.220         0.116   45.466282
81                    Warheads          0       1        0               0       0  ...     1    0         0         0.093         0.116   39.011898
82        Welch's Fruit Snacks          0       1        0               0       0  ...     0    0         1         0.313         0.313   44.375519
83  Werther's Original Caramel          0       0        1               0       0  ...     1    0         0         0.186         0.267   41.904308
84                    Whoppers          1       0        0               0       0  ...     0    0         1         0.872         0.848   49.524113

[85 rows x 13 columns]
```

#### Output Data:
```
competitorname
100 Grand                     0.732
3 Musketeers                  0.604
Air Heads                     0.906
Almond Joy                    0.465
Baby Ruth                     0.604
                              ...  
Twizzlers                     0.220
Warheads                      0.093
Welch's Fruit Snacks          0.313
Werther's Original Caramel    0.186
Whoppers                      0.872
Name: sugarpercent, Length: 85, dtype: float64
```

- **Elapsed Time**: 0.0074 seconds
- **Memory Used**: 1376.00 KB
