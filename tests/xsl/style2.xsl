<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'
		version="3.0">
  <xsl:template match='child::big'><extract><xsl:apply-templates select="child::*[. eq '123']"/></extract></xsl:template>
  <xsl:template match='child::element'><item><xsl:apply-templates/></item></xsl:template>
  <xsl:template match='child::text()'><xsl:sequence select="."/></xsl:template>
</xsl:stylesheet>
