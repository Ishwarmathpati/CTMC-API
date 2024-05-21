<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Product</name>
   <tag></tag>
   <elementGuidId>0477bb0b-c647-4eab-9b34-576a827b4c78</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;productType\&quot; : {\n    \&quot;id\&quot; : \&quot;24f510c3-f334-4099-94e2-d6224a8eb919\&quot;,\n    \&quot;typeId\&quot; : \&quot;product-type\&quot;\n  },\n  \&quot;categories\&quot; : [ {\n    \&quot;typeId\&quot; : \&quot;category\&quot;,\n    \&quot;id\&quot; : \&quot;24f510c3-f334-4099-94e2-d6224a8eb919\&quot;\n  } ],\n  \&quot;name\&quot; : {\n    \&quot;en\&quot; : \&quot;Some Product\&quot;\n  },\n  \&quot;slug\&quot; : {\n    \&quot;en\&quot; : \&quot;product_slug_\u003crandom-uuid\u003e\&quot;\n  },\n  \&quot;masterVariant\&quot; : {\n    \&quot;sku\&quot; : \&quot;SKU-1\&quot;,\n    \&quot;prices\&quot; : [ {\n      \&quot;value\&quot; : {\n        \&quot;currencyCode\&quot; : \&quot;EUR\&quot;,\n        \&quot;centAmount\&quot; : 4200\n      }\n    } ],\n    \&quot;images\&quot; : [ {\n      \&quot;url\&quot; : \&quot;http://my.custom.cdn.net/master.png\&quot;,\n      \&quot;label\&quot; : \&quot;Master Image\&quot;,\n      \&quot;dimensions\&quot; : {\n        \&quot;w\&quot; : 303,\n        \&quot;h\&quot; : 197\n      }\n    } ]\n  },\n  \&quot;variants\&quot; : [ {\n    \&quot;images\&quot; : [ {\n      \&quot;url\&quot; : \&quot;http://my.custom.cdn.net/variant.png\&quot;,\n      \&quot;label\&quot; : \&quot;Variant Image\&quot;,\n      \&quot;dimensions\&quot; : {\n        \&quot;w\&quot; : 303,\n        \&quot;h\&quot; : 197\n      }\n    } ]\n  } ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>c67084c6-638e-4a70-b0a9-c84e66ace31c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>b347086b-a859-425f-8fee-9ae37161df7d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/products</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>23faab4c-dd9e-4202-bed6-8766fdbe8f88</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>00679329-a07f-498f-9ea3-22c37925a03f</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
