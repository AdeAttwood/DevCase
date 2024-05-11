Feature: Case preserving search and replace

  Scenario: You can search and replace with with a regular expression
    Given Search is 'productid'
    And Replace is 'catalogId'
    And Input is 'function GetProductId(productId)'
    Then Output is 'function GetCatalogId(catalogId)'
