<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'
		version="3.0">
  <xsl:output method="text"/>

  <xsl:strip-space elements="*"/>

  <xsl:param name="p1"/>
  <xsl:param name="p2" select="'default value'"/>
  <xsl:variable name="v1" select="concat('p1=', $p1)"/>

  <xsl:template match="/">
Parameter 1 = "<xsl:sequence select="$p1"/>"
Parameter 2 = "<xsl:sequence select="$p2"/>"
Variable 1 = "<xsl:sequence select="$v1"/>"
  </xsl:template>
</xsl:stylesheet>
